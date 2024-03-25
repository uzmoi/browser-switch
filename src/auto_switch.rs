use serde::{Deserialize, Serialize};
use url::Url;

use crate::url_pattern::UrlPattern;

#[derive(Serialize, Deserialize)]
pub struct MatchRule {
    pub browser: String,
    #[serde(with = "serde_url_pattern")]
    pattern: Option<UrlPattern>,
}

impl MatchRule {
    pub fn is_match(&self, url: &Url) -> bool {
        matches!(self.pattern, Some(ref pattern) if pattern.is_match(url))
    }
}

mod serde_url_pattern {
    use super::*;

    pub fn serialize<S: serde::Serializer>(
        value: &Option<UrlPattern>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        value
            .as_ref()
            .map(|pattern| pattern.to_string())
            .serialize(serializer)
    }
    pub fn deserialize<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<UrlPattern>, D::Error> {
        let pattern = String::deserialize(deserializer)?;
        Ok(UrlPattern::parse(&pattern))
    }
}
