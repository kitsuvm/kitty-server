//! This module defines the [`Screen`] trait and the [`ScreenState`] enum, which represent the different screens in the application.

use iced::{Element, Length, Renderer};
use kitty_theme_iced::theme::Theme;

use crate::Message;

pub mod server_add;
pub mod server_list;

/// The trait that defines a screen in the application.
pub trait Screen {
    /// Returns the element to be displayed in the content area of the screen.
    fn content<'a>(&'a self) -> Element<'a, Message, Theme, Renderer>;

    /// Returns the element to be displayed in the opposite side of the window bar.
    fn window_bar_opposite<'a>(&'a self) -> Option<Element<'a, Message, Theme, Renderer>> {
        None
    }

    /// Returns the element to be displayed in the center of the window bar.
    fn window_bar_center<'a>(&'a self) -> Option<Element<'a, Message, Theme, Renderer>> {
        None
    }

    /// Returns the width of the side content of the window bar, if any.
    fn window_bar_side_width(&self) -> Option<Length> {
        None
    }
}

/// The type of screen in the application.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenType {
    /// The server list screen.
    ServerList,
    /// The server add screen.
    ServerAdd,
}

/// The state of the application screen.
#[derive(Debug, Clone)]
pub enum ScreenState {
    /// The server list screen.
    ServerList(server_list::State),
    /// The server add screen.
    ServerAdd(server_add::State),
}

impl ScreenState {
    /// Creates a new `ScreenState` with the default screen.
    pub fn new() -> Self {
        Self::ServerList(server_list::State::new())
    }

    /// Sets the search query for the current screen state.
    pub fn set_search_query(&mut self, query: String) {
        match self {
            Self::ServerList(state) => state.search_query = query,
            _ => {}
        }
    }
}

impl Screen for ScreenState {
    fn content<'a>(&'a self) -> Element<'a, Message, Theme, Renderer> {
        match self {
            Self::ServerList(state) => state.content(),
            Self::ServerAdd(state) => state.content(),
        }
    }

    fn window_bar_opposite<'a>(&'a self) -> Option<Element<'a, Message, Theme, Renderer>> {
        match self {
            Self::ServerList(state) => state.window_bar_opposite(),
            _ => None,
        }
    }

    fn window_bar_center<'a>(&'a self) -> Option<Element<'a, Message, Theme, Renderer>> {
        match self {
            Self::ServerList(state) => state.window_bar_center(),
            _ => None,
        }
    }

    fn window_bar_side_width(&self) -> Option<Length> {
        match self {
            Self::ServerList(state) => state.window_bar_side_width(),
            _ => None,
        }
    }
}

impl From<ScreenType> for ScreenState {
    fn from(screen_type: ScreenType) -> Self {
        match screen_type {
            ScreenType::ServerList => Self::ServerList(server_list::State::new()),
            ScreenType::ServerAdd => Self::ServerAdd(server_add::State::new()),
        }
    }
}
