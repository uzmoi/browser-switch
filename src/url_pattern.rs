use once_cell::sync::Lazy;
use regex::Regex;
use url::{Host, Url};

pub struct UrlPattern {
    scheme: SchemePattern,
    host: HostPattern,
    port: Option<u16>,
}

static RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r#"^(?:(?<scheme>\*|[[:alpha:]]+)://)?(?<host>[*.[:word:]]+)(?<port>:\d+)?(?<path>/.+)?$"#,
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
            .and_then(|port| port.as_str().parse().ok());

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

enum SchemePattern {
    Any,
    HttpOrHttps,
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

enum HostPattern {
    Any,
    SubDomain(String),
    Localhost,
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
    fn is_match(&self, host: Host<&str>) -> bool {
        match self {
            HostPattern::Any => true,
            HostPattern::SubDomain(ref domain_pattern) => {
                debug_assert!(domain_pattern.starts_with('.'));
                matches!(host, Host::Domain(domain) if {
                    domain == &domain_pattern[1..] || domain.ends_with(domain_pattern)
                })
            }
            HostPattern::Localhost => is_localhost(&host),
            HostPattern::Exact(ref host_pattern) => &host == host_pattern,
        }
    }
}

fn is_localhost(host: &Host<impl ToString>) -> bool {
    match host {
        Host::Domain(domain) => domain.to_string() == "localhost",
        Host::Ipv4(address) => address.is_loopback(),
        Host::Ipv6(address) => address.is_loopback(),
    }
}
