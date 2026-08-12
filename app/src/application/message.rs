use iced::Task;
use kitty_theme_iced::window_event;

use crate::{
    application::{
        modal::{ModalKind, ModalState},
        screen::{Screen, ScreenState},
        state::State,
    },
    config::servers::{Servers, ServersState},
};

/// The messages of the application.
#[derive(Debug, Clone)]
pub enum Message {
    /// The window needs to be dragged.
    Window(window_event::Event),
    /// The search input has changed.
    ChangedTextInput(usize, String),
    // /// The screen needs to be changed.
    // ChangeScreen(ScreenKind),
    /// The modal needs to be opened.
    OpenModal(ModalKind),
    /// Close the modal.
    CloseModal,
    /// Submit the modal, closing it.
    SubmitModal,
    /// The connection configuration has been updated.
    ServersUpdate(ServersState),
}

/// Updates the state of the application.
pub fn update(state: &mut State, message: Message) -> Task<Message> {
    tracing::debug!(?message, "Processing message...");
    match message {
        Message::Window(event) => {
            window_event::update(&mut state.window_state, event).map(Message::Window)
        }
        Message::ChangedTextInput(id, query) => {
            if state.modal.is_active() {
                state.modal.handle_text_input(id, query);
            } else {
                state.screen.handle_text_input(id, query);
            }
            Task::none()
        }
        // Message::ChangeScreen(screen_type) => {
        //     state.screen = screen_type.into();
        //     Task::none()
        // }
        Message::OpenModal(modal_kind) => {
            state.modal = modal_kind.into();
            Task::none()
        }
        Message::CloseModal => {
            state.modal = ModalState::None;
            Task::none()
        }
        Message::SubmitModal => {
            match &mut state.modal {
                ModalState::ServerAdd(modal) => {
                    if modal.host.is_empty() {
                        tracing::warn!("Host is empty, cannot submit modal");
                        modal.inputted_host = true;
                        return Task::none();
                    }

                    let mut servers =
                        Servers::load_from_project_dirs(&state.global_state.project_dirs)
                            .unwrap_or_default();

                    servers.ssh_servers.push(modal.as_ref().into());

                    let current_servers = match &mut state.screen {
                        ScreenState::ServerList(state) => match &state.servers_state {
                            ServersState::Data(servers) => Some(servers.clone()),
                            _ => None,
                        },
                    };

                    let servers_state = match servers
                        .save_to_project_dirs(&state.global_state.project_dirs)
                    {
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

                    match &mut state.screen {
                        ScreenState::ServerList(state) => {
                            state.servers_state = servers_state;
                        }
                    }
                }
                _ => {
                    tracing::warn!(modal = ?state.modal, "Message::SubmitModal called for modal that does not support submission");
                }
            }

            state.modal = ModalState::None;
            Task::none()
        }
        Message::ServersUpdate(servers) => {
            match &mut state.screen {
                ScreenState::ServerList(state) => {
                    state.servers_state = servers;
                }
            }
            Task::none()
        }
    }
}
