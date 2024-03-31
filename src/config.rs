use std::{
    fs::File,
    io::{self, Read},
};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use toml_edit::{de::from_document, DocumentMut};
use url::Url;

use crate::{auto_switch::AutoSwitchRule, browser::Browser};

const CONFIG_FILE_NAME: &str = "browser-switch.toml";

pub struct ConfigFile {
    _file: File,
    doc: DocumentMut,
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
        let doc = s
            .parse::<DocumentMut>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
        Ok(ConfigFile { _file: file, doc })
    }
    pub fn to_config(&self) -> Result<Config, toml_edit::de::Error> {
        from_document(self.doc.to_owned())
    }
}

#[derive(Serialize, Deserialize, Default)]
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
