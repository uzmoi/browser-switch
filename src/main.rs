mod auto_switch;
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
    current_url: Option<Url>,
    config: Option<Config>,
}

impl App {
    fn next(&mut self) -> Option<Url> {
        let config = self.config.as_ref()?;

        while let Some(next_url) = self.urls.next() {
            if let Some(browser) = config.match_browser(&next_url) {
                browser.open(vec![next_url.to_string()]);
            } else {
                return Some(next_url);
            }
        }

        None
    }
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

        let config = config::Config::load_file().transpose().ok().flatten();
        let mut app = App {
            urls: urls.into_iter(),
            current_url: None,
            config,
        };
        let command = if let Some(next_url) = app.next() {
            app.current_url = Some(next_url);
            Command::none()
        } else {
            window::close()
        };

        (app, command)
    }
    fn title(&self) -> String {
        String::from("Browser switch")
    }
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Open(browser) => {
                if let Some(current_url) = &self.current_url {
                    browser.open(vec![current_url.to_string()]);
                }
                self.update(Message::Next)
            }
            Message::Next => {
                if let Some(next_url) = self.next() {
                    self.current_url = Some(next_url);
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
            text(format!("URL: {}", self.current_url.as_ref().unwrap())),
            scrollable(browser_list),
            button(text("Cancel")).on_press(Message::Next)
        ])
        .into()
    }
}

fn main() -> iced::Result {
    App::run(Settings::default())
}
