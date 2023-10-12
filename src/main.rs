mod browser;
mod config;

use browser::Browser;
use config::Config;
use iced::{
    widget::{button, column, container, row, scrollable, text, Column},
    window, Application, Command, Settings,
};
use url::Url;

struct App {
    urls: std::vec::IntoIter<Url>,
    current_url: Url,
    config: Option<Config>,
}

#[derive(Debug, Clone)]
enum Message {
    Open(Browser),
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
        let config = config::Config::load_file().transpose().ok().flatten();
        (
            App {
                urls,
                current_url,
                config,
            },
            Command::none(),
        )
    }
    fn title(&self) -> String {
        String::from("Browser switch")
    }
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Open(browser) => {
                browser.open(vec![self.current_url.to_string()]);
                self.update(Message::Next)
            }
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
        let browser_list = Column::with_children(
            self.config
                .as_ref()
                .unwrap()
                .browsers
                .iter()
                .map(|browser| {
                    row![
                        text(&browser.name),
                        button("Open").on_press(Message::Open(browser.clone())),
                    ]
                    .into()
                })
                .collect(),
        );
        container(column![
            text("Browser switch"),
            text(format!("URL: {}", self.current_url)),
            scrollable(browser_list),
            button(text("Cancel")).on_press(Message::Next)
        ])
        .into()
    }
}

fn main() -> iced::Result {
    App::run(Settings::default())
}
