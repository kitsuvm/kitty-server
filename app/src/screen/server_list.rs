//! The server list screen.

use iced::{
    Element, Length, Padding, Renderer,
    alignment::Vertical,
    widget::{container, text, text_input},
};
use kitty_theme_iced::{
    theme::Theme,
    widget::{icon, icon_button},
};

use crate::{GlobalState, Message, modal::ModalKind, screen::Screen, servers::ServersState};

/// The state of the server list screen.
#[derive(Debug, Clone, Default)]
pub struct State {
    /// The search query entered by the user.
    pub search_query: String,
    /// The connection state of the application.
    pub servers_state: ServersState,
}

impl Screen for State {
    fn content<'a>(&'a self, _: &GlobalState) -> Element<'a, Message, Theme, Renderer> {
        match &self.servers_state {
            ServersState::Loading => container(text("Loading..."))
                .center(Length::Fill)
                .style(container::transparent)
                .into(),
            ServersState::Data(server_manager) if server_manager.is_empty() => container(
                text("No Clients Available").style(|theme: &Theme| text::Style {
                    color: Some(theme.extended().background.weaker.text),
                }),
            )
            .style(container::transparent)
            .center(Length::Fill)
            .into(),
            ServersState::Data(server_manager) => container(
                text(format!(
                    "{} Clients Available",
                    server_manager.servers.ssh_servers.len()
                ))
                .style(|theme: &Theme| text::Style {
                    color: Some(theme.extended().background.weaker.text),
                }),
            )
            .style(container::transparent)
            .center(Length::Fill)
            .into(),
            ServersState::Error(e) => container(text(format!("Error: {}", e)).style(
                |theme: &Theme| text::Style {
                    color: Some(theme.extended().background.weaker.text),
                },
            ))
            .style(container::transparent)
            .center(Length::Fill)
            .into(),
        }
    }

    fn handle_text_input(&mut self, _id: usize, value: String) {
        self.search_query = value;
    }

    fn window_bar_side_width(&self) -> Option<Length> {
        Some(120.into())
    }

    fn window_bar_opposite<'a>(&'a self) -> Option<Element<'a, Message, Theme, Renderer>> {
        Some(
            container(
                icon_button(icon::ADD_ICON).on_press(Message::OpenModal(ModalKind::ServerAdd)),
            )
            .style(container::transparent)
            .padding(Padding::from(0).left(8))
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
                    .on_input(|v| Message::ChangedTextInput(0, v)),
            )
            .max_width(360)
            .padding(Padding::from(0).top(8))
            .into(),
        )
    }
}
