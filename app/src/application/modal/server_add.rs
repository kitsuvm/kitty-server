//! The server add modal.

use i18n_embed_fl::fl;
use iced::{
    Element, Length, Renderer, Task,
    widget::{column, row, space, text, text_input},
};
use kitty_theme_iced::{BaseExtended, theme::Theme, widget::button};

use crate::{
    application::{message::Message, modal::Modal, screen::ScreenState, state::GlobalState},
    config::servers::{SSHServer, Servers, ServersState},
};

/// The state of the server list screen.
#[derive(Debug, Clone, Default)]
pub struct State {
    /// The host of the server to be added.
    pub host: String,
    /// Whether the user has inputted a host.
    pub inputted_host: bool,
    /// The port of the server to be added.
    pub port: String,
    /// The username of the server to be added.
    pub username: String,
    /// The name of the server to be added.
    pub name: String,
}

impl AsRef<State> for State {
    fn as_ref(&self) -> &State {
        self
    }
}

impl From<&State> for SSHServer {
    fn from(state: &State) -> Self {
        SSHServer {
            host: state.host.clone(),
            port: state.port.parse().ok(),
            username: if !state.username.is_empty() {
                Some(state.username.clone())
            } else {
                None
            },
            name: if !state.name.is_empty() {
                Some(state.name.clone())
            } else {
                None
            },
        }
    }
}

impl Modal for State {
    fn content<'a>(&'a self, global_state: &GlobalState) -> Element<'a, Message, Theme, Renderer> {
        column![
            text(fl!(global_state.i18n, "server-name")),
            text_input(&fl!(global_state.i18n, "server-name-example"), &self.name)
                .on_input(|v| Message::ChangedTextInput(3, v)),
            text(fl!(global_state.i18n, "host")),
            text_input(&fl!(global_state.i18n, "host-example"), &self.host)
                .on_input(|v| Message::ChangedTextInput(0, v)),
            if self.host.is_empty() && self.inputted_host {
                Some(
                    text(fl!(global_state.i18n, "host-empty-error")).style(|theme: &Theme| {
                        text::Style {
                            color: Some(theme.palette_extended().danger.base.color),
                        }
                    }),
                )
            } else {
                None
            },
            text(fl!(global_state.i18n, "port")),
            text_input(&fl!(global_state.i18n, "port-example"), &self.port).on_input(|v| {
                let port = v.parse::<u16>();
                if port.is_ok() || v.is_empty() {
                    Message::ChangedTextInput(1, v)
                } else {
                    Message::ChangedTextInput(1, self.port.clone())
                }
            }),
            text(fl!(global_state.i18n, "username")),
            text_input(&fl!(global_state.i18n, "username-example"), &self.username)
                .on_input(|v| Message::ChangedTextInput(2, v)),
            space().height(10),
            row![
                button(text(fl!(global_state.i18n, "close")).center())
                    .padding(6)
                    .on_press(Message::CloseModal)
                    .width(Length::Fill),
                button(text(fl!(global_state.i18n, "add")).center())
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
            3 => self.name = value,
            _ => {}
        }
    }

    fn handle_submit(
        &mut self,
        global_state: &mut GlobalState,
        screen: &mut ScreenState,
    ) -> (bool, Task<Message>) {
        if self.host.is_empty() {
            tracing::warn!("Host is empty, cannot submit modal");
            self.inputted_host = true;
            return (false, Task::none());
        }

        let mut servers =
            Servers::load_from_project_dirs(&global_state.project_dirs).unwrap_or_default();

        servers.ssh_servers.push(self.as_ref().into());

        let current_servers = match screen {
            ScreenState::ServerList(state) => match &state.servers_state {
                ServersState::Data(servers) => Some(servers.clone()),
                _ => None,
            },
        };

        let servers_state = match servers.save_to_project_dirs(&global_state.project_dirs) {
            Ok(_) => {
                tracing::info!("Saved connection configuration file, reloading...");
                ServersState::Data(match current_servers {
                    Some(mut v) => {
                        v.servers = servers;
                        v
                    }
                    None => servers.into(),
                })
            }
            Err(e) => {
                tracing::error!("Could not save connection configuration file: {}", e);
                ServersState::Error(e)
            }
        };

        match screen {
            ScreenState::ServerList(state) => {
                state.servers_state = servers_state;
            }
        };

        (true, Task::none())
    }
}
