use crate::{auto_switch::MatchRule, browser::Browser};
use url::Url;

static CONFIG_FILE_NAME: &str = "browser-switch.json";

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct Config {
    pub browsers: Vec<Browser>,
    rules: Vec<MatchRule>,
}

impl Config {
    pub fn load_file() -> Option<serde_json::Result<Config>> {
        std::fs::read(CONFIG_FILE_NAME).ok().map(|ref config_file| {
            let config = serde_json::from_slice::<Config>(config_file);
            config
        })
    }
    pub fn match_browser(&self, url: &Url) -> Option<Browser> {
        for rule in self.rules.iter() {
            if rule.is_match(url) {
                if let Some(browser) = self
                    .browsers
                    .iter()
                    .find(|browser| browser.name == rule.browser)
                {
                    return Some(browser.clone());
                }
            }
        }
        None
    }
}
