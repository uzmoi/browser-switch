use iced::{
    alignment::Vertical,
    widget::{button, horizontal_space, image, row, text},
    ContentFit, Element, Length,
};

use crate::app::Message;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Browser {
    pub name: String,
    command: String,
    pub icon: Option<String>,
}

impl Browser {
    pub fn open(self, args: Vec<String>) {
        std::process::Command::new(self.command)
            .args(args)
            .spawn()
            .expect("Failed to open browser.");
    }
    pub fn view_browser<'a>(&self) -> Element<'a, Message> {
        static ICON_SIZE: u16 = 32;
        let icon: Element<'_, Message> = if let Some(icon) = &self.icon {
            image(icon)
                .content_fit(ContentFit::Fill)
                .height(ICON_SIZE)
                .width(ICON_SIZE)
                .into()
        } else {
            horizontal_space().width(ICON_SIZE).into()
        };
        row![
            icon,
            text(&self.name)
                .width(Length::Fill)
                .vertical_alignment(Vertical::Center),
            button("Open").on_press(Message::Open(self.clone())),
        ]
        .spacing(8)
        .height(32)
        .into()
    }
}
