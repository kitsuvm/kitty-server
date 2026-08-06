use iced::{
    Element, Renderer,
    widget::{button, column, scrollable, text_input},
};
use kitty_theme_iced::{
    theme::Theme,
    widget::{content, icon},
};

use crate::Message;

pub struct State {
    search_query: String,
}

impl State {
    pub fn new() -> Self {
        Self {
            search_query: String::new(),
        }
    }

    pub fn window_bar_opposite<'a>(&'a self) -> Element<'a, Message, Theme, Renderer> {
        button(icon(icon::ADD_ICON)).into()
    }

    pub fn window_bar_center<'a>(&'a self) -> Element<'a, Message, Theme, Renderer> {
        text_input("Search...", &self.search_query)
            .icon(icon::to_text_input_icon(icon::SEARCH_ICON, 0.0, None))
            .into()
    }

    pub fn content<'a>(&'a self) -> Element<'a, Message, Theme, Renderer> {
        content(scrollable(column![])).padding(0).into()
    }
}
