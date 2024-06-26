use iced::{
    widget::{button, horizontal_space, image, row, text},
    Alignment, ContentFit, Element, Length,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Browser {
    name: String,
    command: String,
    icon: Option<String>,
}

impl Browser {
    pub fn open(&self, args: Vec<String>) {
        std::process::Command::new(&self.command)
            .args(args)
            .spawn()
            .expect("Failed to open browser.");
    }
    pub fn view_browser<'a, Message: Clone + 'a>(&self, on_press: Message) -> Element<'a, Message> {
        const ICON_SIZE: u16 = 32;

        let icon: Element<'_, Message> = if let Some(icon) = &self.icon {
            image(icon)
                .content_fit(ContentFit::Fill)
                .height(ICON_SIZE)
                .width(ICON_SIZE)
                .into()
        } else {
            horizontal_space().height(ICON_SIZE).width(ICON_SIZE).into()
        };

        button(
            row![icon, text(&self.name).width(Length::Fill)]
                .spacing(8)
                .align_items(Alignment::Center)
                .padding(4),
        )
        .on_press(on_press)
        .into()
    }
}
