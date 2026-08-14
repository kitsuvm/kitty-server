//! The server add modal.

use iced::{
    Element, Length, Renderer, Task,
    widget::{column, row, space, text_input},
};
use kitty_theme_iced::{
    BaseExtended,
    theme::Theme,
    widget::{button, text},
};
use whoami::username;

use crate::{
    application::{
        message::Message,
        modal::Modal,
        screen::ScreenState,
        state::{GlobalState, Lazy},
    },
    resources::hosts::{HostsManager, SshHost},
    t,
};

/// The state of the server list screen.
#[derive(Debug, Clone)]
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
    /// The current username of the user.
    pub current_username: Option<String>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            host: String::new(),
            inputted_host: false,
            port: String::new(),
            username: String::new(),
            name: String::new(),
            current_username: username()
                .inspect_err(|e| {
                    tracing::error!(?e, "Failed to get current username");
                })
                .ok(),
        }
    }
}

impl AsRef<State> for State {
    fn as_ref(&self) -> &State {
        self
    }
}

impl From<&State> for SshHost {
    fn from(state: &State) -> Self {
        SshHost {
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
            text(t!(global_state, "server-name")),
            text_input(&t!(global_state, "server-name", "example"), &self.name)
                .on_input(|v| Message::ChangedTextInput(3, v)),
            text(t!(global_state, "host")),
            text_input(&t!(global_state, "host", "example"), &self.host)
                .on_input(|v| Message::ChangedTextInput(0, v)),
            if self.host.is_empty() && self.inputted_host {
                Some(
                    text(t!(global_state, "host", "empty-error")).style(|theme: &Theme| {
                        text::Style {
                            color: Some(theme.palette_extended().danger.base.color),
                        }
                    }),
                )
            } else {
                None
            },
            text(t!(global_state, "port")),
            text_input(&t!(global_state, "port", "example"), &self.port).on_input(|v| {
                let port = v.parse::<u16>();
                if port.is_ok() || v.is_empty() {
                    Message::ChangedTextInput(1, v)
                } else {
                    Message::ChangedTextInput(1, self.port.clone())
                }
            }),
            text(t!(global_state, "username")),
            text_input(
                self.current_username
                    .as_ref()
                    .unwrap_or(&t!(global_state, "username", "example")),
                &self.username
            )
            .on_input(|v| Message::ChangedTextInput(2, v)),
            space().height(10),
            row![
                button(
                    text(t!(global_state, "close"))
                        .style(text::default)
                        .center()
                )
                .padding(6)
                .on_press(Message::CloseModal)
                .width(Length::Fill),
                button(text(t!(global_state, "add")).style(text::default).center())
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

        let mut hosts = HostsManager::new(&global_state.resource_manager);

        if let Err(e) = hosts.push(&global_state.resource_manager, self.as_ref().into()) {
            tracing::error!(?e, "Failed to save host to disk");
        }

        match screen {
            ScreenState::ServerList(state) => {
                state.internal = Lazy::Data(hosts);
            }
        }

        (true, Task::none())
    }
}
