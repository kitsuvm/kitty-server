//! The server add modal.

use iced::{Element, Renderer, widget::space};
use kitty_theme_iced::theme::Theme;

use crate::{Message, modal::Modal};

/// The state of the server list screen.
#[derive(Debug, Clone, Default)]
pub struct State {}

impl Modal for State {
    fn content<'a>(&'a self) -> Element<'a, Message, Theme, Renderer> {
        space().into()
    }
}
