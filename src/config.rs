use std::{
    fs::File,
    io::{self, Read},
};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{auto_switch::AutoSwitchRule, browser::Browser};

static CONFIG_FILE_NAME: &str = "browser-switch.json";

pub struct ConfigFile {
    _file: File,
    config: Config,
}

impl ConfigFile {
    fn open() -> io::Result<File> {
        File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(CONFIG_FILE_NAME)
    }
    pub fn load() -> io::Result<ConfigFile> {
        let mut file = Self::open()?;
        let mut s = String::new();
        file.read_to_string(&mut s)?;
        let config = serde_json::from_str(&s)?;
        Ok(ConfigFile {
            _file: file,
            config,
        })
    }
    pub fn to_config(&self) -> serde_json::Result<Config> {
        Ok(self.config.to_owned())
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Config {
    #[serde(default)]
    pub always_on_top: bool,
    pub browsers: IndexMap<String, Browser>,
    rules: Vec<AutoSwitchRule>,
}

impl Config {
    pub fn match_browser(&self, url: &Url) -> Option<&Browser> {
        let rule = self.rules.iter().find(|rule| rule.is_match(url))?;
        self.browsers.get(&rule.browser)
    }
}
