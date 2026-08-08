//! The server list screen.

use iced::{
    Element, Length, Padding, Renderer,
    alignment::Vertical,
    widget::{container, row, space, text, text_input},
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
pub struct State {
    /// The search query entered by the user.
    pub search_query: String,
}

impl State {
    /// Creates a new [`State`] with an empty search query.
    pub fn new() -> Self {
        Self {
            search_query: String::new(),
        }
    }
}

impl Screen for State {
    fn content<'a>(&'a self) -> Element<'a, Message, Theme, Renderer> {
        container(
            text("No Clients Available").style(|theme: &Theme| text::Style {
                color: Some(theme.extended().background.weaker.text),
            }),
        )
        .style(container::transparent)
        .center(Length::Fill)
        .into()
    }

    fn window_bar_side_width(&self) -> Option<Length> {
        Some(120.into())
    }

    fn window_bar_opposite<'a>(&'a self) -> Option<Element<'a, Message, Theme, Renderer>> {
        Some(
            row![
                space().width(8),
                icon_button(icon::ADD_ICON).on_press(Message::ChangeScreen(ScreenType::ServerAdd))
            ]
            .height(Length::Fill)
            .align_y(Vertical::Bottom)
            .into(),
        )
    }

    fn window_bar_center<'a>(&'a self) -> Option<Element<'a, Message, Theme, Renderer>> {
        Some(
            container(
                text_input("Search...", &self.search_query)
                    .icon(icon::to_text_input_icon(icon::SEARCH_ICON, 0.0, None))
                    .on_input(Message::SearchInputChanged),
            )
            .max_width(360)
            .padding(Padding::from(0).top(8))
            .into(),
        )
    }
}
