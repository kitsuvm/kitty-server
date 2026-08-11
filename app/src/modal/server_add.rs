//! The server add modal.

use iced::{
    Element, Length, Renderer,
    widget::{column, row, space, text, text_input},
};
use kitty_theme_iced::{BaseExtended, theme::Theme, widget::button};

use crate::{Message, modal::Modal};

/// The state of the server list screen.
#[derive(Debug, Clone, Default)]
pub struct State {
    pub host: String,
    pub inputted_host: bool,
    pub port: String,
    pub username: String,
}

impl Modal for State {
    fn content<'a>(&'a self) -> Element<'a, Message, Theme, Renderer> {
        column![
            text("Host"),
            text_input("127.0.0.1", &self.host).on_input(|v| Message::ChangedTextInput(0, v)),
            if self.host.is_empty() && self.inputted_host {
                Some(
                    text("Please enter a host").style(|theme: &Theme| text::Style {
                        color: Some(theme.palette_extended().danger.base.color),
                    }),
                )
            } else {
                None
            },
            text("Port"),
            text_input("22", &self.port).on_input(|v| Message::ChangedTextInput(1, v)),
            text("Username"),
            text_input("root", &self.username).on_input(|v| Message::ChangedTextInput(2, v)),
            space().height(10),
            row![
                button(text("Close").center())
                    .padding(6)
                    .on_press(Message::CloseModal)
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

    fn handle_text_input(&mut self, id: usize, value: String) {
        match id {
            0 => {
                self.host = value;
                self.inputted_host = true;
            }
            1 => self.port = value,
            2 => self.username = value,
            _ => {}
        }
    }
}
