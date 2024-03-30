mod app;
mod auto_switch;
mod browser;
mod config;
mod url_pattern;

use app::App;
use config::ConfigFile;
use iced::{window, Application};

fn main() -> iced::Result {
    let config_file = ConfigFile::load().ok();

    let config = config_file
        .as_ref()
        .and_then(|config_file| config_file.to_config().ok());
    let level = match config {
        Some(ref config) if config.always_on_top => window::Level::AlwaysOnTop,
        _ => window::Level::Normal,
    };

    App::run(iced::Settings {
        window: window::Settings {
            level,
            ..window::Settings::default()
        },
        flags: config_file,
        ..iced::Settings::default()
    })
}
