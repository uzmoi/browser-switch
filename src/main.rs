mod app;
mod auto_switch;
mod browser;
mod config;
mod url_pattern;

use app::App;
use config::Config;
use iced::{window, Application};

fn main() -> iced::Result {
    let config = Config::load_file().ok();

    let level = match config {
        Some(ref config) if config.always_on_top => window::Level::AlwaysOnTop,
        _ => window::Level::Normal,
    };

    App::run(iced::Settings {
        window: window::Settings {
            level,
            ..window::Settings::default()
        },
        flags: config,
        ..iced::Settings::default()
    })
}
