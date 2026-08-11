//! The server add modal.

use iced::{
    Element, Length, Padding, Renderer,
    widget::{column, container, row, space, text, text_input},
};
use kitty_theme_iced::{
    theme::Theme,
    widget::{button, icon, icon_button},
};

use crate::{Message, modal::Modal};

/// The state of the server list screen.
#[derive(Debug, Clone, Default)]
pub struct State {
    pub username: String,
    pub host: String,
    pub port: String,
}

impl Modal for State {
    fn content<'a>(&'a self) -> Element<'a, Message, Theme, Renderer> {
        column![
            text("Host"),
            text_input("127.0.0.1", &self.host),
            text("Port"),
            text_input("22", &self.port),
            text("Username"),
            text_input("root", &self.username),
            space().height(10),
            row![
                button(text("Close").center())
                    .padding(6)
                    .on_press(Message::SubmitModal)
                    .width(Length::Fill),
                button(text("Add").center())
                    .padding(6)
                    .on_press(Message::SubmitModal)
                    .width(Length::Fill)
                    .style(button::primary),
            ]
            .spacing(8)
        ]
        .spacing(6)
        .into()
    }
}
