use crate::browser::Browser;

static CONFIG_FILE_NAME: &str = "browser-switch.json";

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct Config {
    pub browsers: Vec<Browser>,
}

impl Config {
    pub fn load_file() -> Option<serde_json::Result<Config>> {
        std::fs::read(CONFIG_FILE_NAME).ok().map(|ref config_file| {
            let config = serde_json::from_slice::<Config>(config_file);
            config
        })
    }
}
