use iced::{Element, Renderer};
use kitty_theme_iced::theme::Theme;

use crate::Message;

pub mod server_list;

pub enum Screen {
    ServerList(server_list::State),
}

impl Screen {
    pub fn new() -> Self {
        Self::ServerList(server_list::State::new())
    }

    pub fn window_bar_opposite<'a>(&'a self) -> Element<'a, Message, Theme, Renderer> {
        match self {
            Self::ServerList(state) => state.window_bar_opposite(),
        }
    }

    pub fn window_bar_center<'a>(&'a self) -> Element<'a, Message, Theme, Renderer> {
        match self {
            Self::ServerList(state) => state.window_bar_center(),
        }
    }

    pub fn content<'a>(&'a self) -> Element<'a, Message, Theme, Renderer> {
        match self {
            Self::ServerList(state) => state.content(),
        }
    }
}
