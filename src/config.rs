use std::{fs::File, io};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{auto_switch::AutoSwitchRule, browser::Browser};

static CONFIG_FILE_NAME: &str = "browser-switch.json";

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub always_on_top: bool,
    pub browsers: IndexMap<String, Browser>,
    rules: Vec<AutoSwitchRule>,
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
