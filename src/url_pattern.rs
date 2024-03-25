use std::fmt;

use once_cell::sync::Lazy;
use parse_display::Display;
use regex::Regex;
use url::{Host, Url};

#[derive(Default, PartialEq, Debug)]
pub struct UrlPattern {
    scheme: SchemePattern,
    host: HostPattern,
    port: Option<u16>,
}

static RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r#"^(?:(?<scheme>\*|[[:alpha:]]+\??)://)?(?<host>[*.[:word:]]+)(?<port>:[0-9]+)?(?<path>/.+)?$"#,
    )
    .unwrap()
});

impl UrlPattern {
    pub fn parse(str: &str) -> Option<UrlPattern> {
        let captures = RE.captures(str)?;

        let scheme = match captures.name("scheme").map(|m| m.as_str()) {
            Some("*") | None => SchemePattern::Any,
            Some("https?") => SchemePattern::HttpOrHttps,
            Some(scheme) => SchemePattern::Exact(scheme.to_owned()),
        };

        let host = HostPattern::parse(&captures["host"]);

        let port = captures
            .name("port")
            .and_then(|m| m.as_str()[1..].parse().ok());

        Some(UrlPattern { scheme, host, port })
    }
    pub fn is_match(&self, url: &Url) -> bool {
        self.scheme.is_match(url.scheme())
            && url.host().map_or(true, |host| {
                self.host.is_match(host) && self.is_match_port(url.port_or_known_default())
            })

        // TODO: url.path(), url.query_pairs(), url.fragment();
    }
    fn is_match_port(&self, port: Option<u16>) -> bool {
        match (self.port, port) {
            (None, _) => true,
            (Some(port_pattern), Some(port)) => port == port_pattern,
            (Some(_), None) => false,
        }
    }
}

impl fmt::Display for UrlPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if matches!(self.scheme, SchemePattern::Any) {
            write!(f, "{}://", self.scheme)?;
        }
        write!(f, "{}", self.host)?;
        if let Some(port) = self.port {
            write!(f, ":{port}")?;
        }
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Display)]
enum SchemePattern {
    #[default]
    #[display("*")]
    Any,
    #[display("https?")]
    HttpOrHttps,
    #[display("{0}")]
    Exact(String),
}

impl SchemePattern {
    fn is_match(&self, scheme: &str) -> bool {
        match self {
            SchemePattern::Any => true,
            SchemePattern::HttpOrHttps => matches!(scheme, "http" | "https"),
            SchemePattern::Exact(ref scheme_pattern) => scheme == scheme_pattern,
        }
    }
}

#[derive(Default, PartialEq, Debug, Display)]
enum HostPattern {
    #[default]
    #[display("*")]
    Any,
    #[display("*{0}")]
    SubDomain(String),
    #[display("localhost")]
    Localhost,
    #[display("{0}")]
    Exact(Host),
}

impl HostPattern {
    fn parse(host_pattern: &str) -> HostPattern {
        match host_pattern {
            "*" => HostPattern::Any,
            host_pattern if host_pattern.starts_with("*.") => {
                HostPattern::SubDomain(host_pattern[1..].to_string())
            }
            host_pattern => match Host::parse(host_pattern) {
                Ok(host_pattern) if is_localhost(&host_pattern) => HostPattern::Localhost,
                Ok(host_pattern) => HostPattern::Exact(host_pattern),
                Err(_) => HostPattern::Any,
            },
        }
    }
    fn is_match(&self, host: Host<impl AsRef<str> + PartialEq<String>>) -> bool {
        match self {
            HostPattern::Any => true,
            HostPattern::SubDomain(ref domain_pattern) => {
                debug_assert!(domain_pattern.starts_with('.'));
                matches!(host, Host::Domain(domain) if {
                    let domain = domain.as_ref();
                    domain == &domain_pattern[1..] || domain.ends_with(domain_pattern)
                })
            }
            HostPattern::Localhost => is_localhost(&host),
            HostPattern::Exact(ref host_pattern) => &host == host_pattern,
        }
    }
}

fn is_localhost(host: &Host<impl AsRef<str>>) -> bool {
    match host {
        Host::Domain(domain) => domain.as_ref() == "localhost",
        Host::Ipv4(address) => address.is_loopback(),
        Host::Ipv6(address) => address.is_loopback(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!(UrlPattern::parse("*://*").unwrap(), UrlPattern::default());

        let pattern = UrlPattern::parse("https?://*").unwrap();
        assert_eq!(pattern.scheme, SchemePattern::HttpOrHttps);

        assert_eq!(
            UrlPattern::parse("https://*.example.com:80").unwrap(),
            UrlPattern {
                scheme: SchemePattern::Exact("https".to_owned()),
                host: HostPattern::SubDomain(".example.com".to_owned()),
                port: Some(80)
            }
        );
    }

    #[test]
    fn match_any_scheme() {
        assert!(SchemePattern::Any.is_match("foo"));
    }

    #[test]
    fn match_http_or_https_scheme() {
        assert!(SchemePattern::HttpOrHttps.is_match("http"));
        assert!(SchemePattern::HttpOrHttps.is_match("https"));
        assert!(!SchemePattern::HttpOrHttps.is_match("file"));
    }

    #[test]
    fn match_exact_scheme() {
        let pattern = SchemePattern::Exact("http".to_owned());
        assert!(pattern.is_match("http"));
        assert!(!pattern.is_match("https"));
    }

    #[test]
    fn parse_host() {
        assert_eq!(HostPattern::parse("*"), HostPattern::Any);
        assert_eq!(
            HostPattern::parse("*.example.com"),
            HostPattern::SubDomain(".example.com".to_owned())
        );
        assert_eq!(HostPattern::parse("localhost"), HostPattern::Localhost);
        assert_eq!(HostPattern::parse("127.0.0.1"), HostPattern::Localhost);
        assert_eq!(HostPattern::parse("[::1]"), HostPattern::Localhost);
        assert_eq!(
            HostPattern::parse("example.com"),
            HostPattern::Exact(Host::parse("example.com").unwrap())
        );
    }

    impl HostPattern {
        fn test(&self, s: &str) -> bool {
            self.is_match(Host::parse(s).unwrap())
        }
    }

    #[test]
    fn match_any_host() {
        assert!(HostPattern::Any.test("example.com"));
    }

    #[test]
    fn match_subdomain() {
        let pattern = HostPattern::SubDomain(".example.com".to_string());
        assert!(pattern.test("example.com"));
        assert!(pattern.test("foo.example.com"));
        assert!(pattern.test("bar.foo.example.com"));
        assert!(!pattern.test("example.net"));
    }

    #[test]
    fn match_localhost() {
        assert!(HostPattern::Localhost.test("localhost"));
        assert!(HostPattern::Localhost.test("127.0.0.1"));
        assert!(HostPattern::Localhost.test("[::1]"));
    }

    #[test]
    fn match_exact_host() {
        let host = Host::parse("example.com").unwrap();
        let pattern = HostPattern::Exact(host.clone());
        assert!(pattern.is_match(host));
        assert!(!pattern.test("foo.example.com"));
    }
}
