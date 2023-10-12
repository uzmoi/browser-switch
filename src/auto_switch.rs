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
        println!("url: {url}, host: {:?}", url.host());
        if let (Some(a), Some(b)) = (url.host(), &self.host) {
            return &a.to_string() == b;
        } else {
            false
        }
    }
}
