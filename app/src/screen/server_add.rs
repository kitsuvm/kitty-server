//! The server list screen.

use iced::{
    Element, Length, Renderer,
    alignment::Vertical,
    widget::{row, space},
};
use kitty_theme_iced::{
    theme::Theme,
    widget::{icon, icon_button},
};

use crate::{
    Message,
    screen::{Screen, ScreenType},
};

/// The state of the server list screen.
#[derive(Debug, Clone)]
pub struct State {}

impl State {
    /// Creates a new [`State`] with an empty search query.
    pub fn new() -> Self {
        Self {}
    }
}

impl Screen for State {
    fn content<'a>(&'a self) -> Element<'a, Message, Theme, Renderer> {
        space().into()
    }

    fn window_bar_opposite<'a>(&'a self) -> Option<Element<'a, Message, Theme, Renderer>> {
        Some(
            row![
                space().width(8),
                icon_button(icon::CHEVRON_LEFT_ICON)
                    .on_press(Message::ChangeScreen(ScreenType::ServerList))
            ]
            .height(Length::Fill)
            .align_y(Vertical::Bottom)
            .into(),
        )
    }
}
