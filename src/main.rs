mod app;
mod auto_switch;
mod browser;
mod config;

use app::App;
use iced::{window, Application};

fn main() -> iced::Result {
    App::run(iced::Settings {
        window: window::Settings {
            level: window::Level::AlwaysOnTop,
            ..window::Settings::default()
        },
        ..iced::Settings::default()
    })
}
