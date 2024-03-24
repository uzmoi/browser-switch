use serde::{Deserialize, Deserializer, Serialize};
use url::Url;

use crate::url_pattern::UrlPattern;

#[derive(Serialize, Deserialize)]
pub struct MatchRule {
    pub browser: String,
    #[serde(skip_serializing, deserialize_with = "deserialize_match_rule")]
    pattern: Option<UrlPattern>,
}

impl MatchRule {
    pub fn is_match(&self, url: &Url) -> bool {
        matches!(self.pattern, Some(ref pattern) if pattern.is_match(url))
    }
}

fn deserialize_match_rule<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<UrlPattern>, D::Error> {
    let pattern = String::deserialize(deserializer)?;
    Ok(UrlPattern::parse(&pattern))
}
