use iced::{
    widget::{button, column, container, text},
    window, Application, Command, Settings,
};
use url::Url;

struct App {
    urls: std::vec::IntoIter<Url>,
    current_url: Url,
}

#[derive(Debug, Clone)]
enum Message {
    Next,
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::theme::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let urls = std::env::args()
            .skip(1)
            .filter_map(|arg| Url::parse(&arg).ok())
            .collect::<Vec<_>>();
        let mut urls = urls.into_iter();
        let current_url = urls.next().unwrap();
        (App { urls, current_url }, Command::none())
    }
    fn title(&self) -> String {
        String::from("Browser switch")
    }
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Next => {
                if let Some(next_url) = self.urls.next() {
                    self.current_url = next_url;
                    Command::none()
                } else {
                    window::close()
                }
            }
        }
    }
    fn view(&self) -> iced::Element<'_, Self::Message> {
        container(column![
            text("Browser switch"),
            text(format!("URL: {}", self.current_url)),
            button(text("Cancel")).on_press(Message::Next)
        ])
        .into()
    }
}

fn main() -> iced::Result {
    App::run(Settings::default())
}
