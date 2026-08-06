//! The server list screen.

use iced::{Element, Renderer, widget::space};
use kitty_theme_iced::theme::Theme;

use crate::{Message, screen::Screen};

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
}
