//! This module defines the [`Screen`] trait and the [`ScreenState`] enum, which represent the different screens in the application.

use iced::{Element, Length, Renderer};
use kitty_theme_iced::theme::Theme;

use crate::{
    application::{message::Message, state::GlobalState},
    impl_screen,
};

pub mod macros;
pub mod server_list;

/// The trait that defines a screen in the application.
pub trait Screen {
    /// Returns the element to be displayed in the content area of the screen.
    fn content<'a>(&'a self, global_state: &GlobalState) -> Element<'a, Message, Theme, Renderer>;

    /// Handles a text input change event for the screen.
    fn handle_text_input(&mut self, _id: usize, _value: String) {}

    /// Refreshes the screen state.
    fn refresh(&mut self) {}

    /// Returns the element to be displayed in the opposite side of the window bar.
    fn window_bar_opposite<'a>(&'a self) -> Option<Element<'a, Message, Theme, Renderer>> {
        None
    }

    /// Returns the element to be displayed in the center of the window bar.
    fn window_bar_center<'a>(
        &'a self,
        _global_state: &GlobalState,
    ) -> Option<Element<'a, Message, Theme, Renderer>> {
        None
    }

    /// Returns the width of the side content of the window bar, if any.
    fn window_bar_side_width(&self) -> Option<Length> {
        None
    }
}

impl_screen! {
    ScreenState, ScreenKind {
        #[default]
        ServerList(server_list::State),
    }
}
