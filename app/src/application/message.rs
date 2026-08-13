use iced::Task;
use kitty_theme_iced::window_event;

use crate::{
    application::{
        modal::{ModalKind, ModalState},
        screen::{Screen, ScreenState},
        state::State,
    },
    config::servers::ServersState,
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
            let (close_modal, task) = state
                .modal
                .handle_submit(&mut state.global_state, &mut state.screen);
            if close_modal {
                state.modal = ModalState::None;
            }
            task
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
