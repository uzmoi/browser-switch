use std::{fs::File, io};

use indexmap::IndexMap;
use url::Url;

use crate::{auto_switch::MatchRule, browser::Browser};

static CONFIG_FILE_NAME: &str = "browser-switch.json";

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct Config {
    pub browsers: IndexMap<String, Browser>,
    rules: Vec<MatchRule>,
}

impl Config {
    pub fn load_file() -> io::Result<Config> {
        let config_file = File::open(CONFIG_FILE_NAME)?;
        let config: Config = serde_json::from_reader(config_file)?;
        Ok(config)
    }
    pub fn match_browser(&self, url: &Url) -> Option<&Browser> {
        let rule = self.rules.iter().find(|rule| rule.is_match(url))?;
        self.browsers.get(&rule.browser)
    }
}
