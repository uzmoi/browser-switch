use url::Url;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct MatchRule {
    pub browser: String,
    regex: Option<String>,
    host: Option<String>,
    port: Option<String>,
}

impl MatchRule {
    pub fn is_match(&self, url: &Url) -> bool {
        if let (Some(host), Some(rule_host)) = (url.host(), &self.host) {
            &host.to_string() == rule_host
        } else {
            false
        }
    }
}
