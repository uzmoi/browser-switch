use iced::{
    widget::{container, text},
    Sandbox, Settings,
};

struct App {}

#[derive(Debug, Clone)]
enum Message {}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        App {}
    }
    fn title(&self) -> String {
        String::from("Browser switch")
    }
    fn update(&mut self, message: Message) {
        match message {}
    }
    fn view(&self) -> iced::Element<'_, Self::Message> {
        container(text("Browser switch")).into()
    }
}

fn main() -> iced::Result {
    App::run(Settings::default())
}
