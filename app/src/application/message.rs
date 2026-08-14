use iced::{Task, system, theme::Mode};
use kitty_theme_iced::window_event;

use crate::{
    application::{
        modal::{ModalKind, ModalState},
        screen::{Screen, ScreenState},
        state::State,
    },
    config::{self, servers::ServersState},
    i18n::change_language,
};

/// The messages of the application.
#[derive(Debug, Clone)]
pub enum Message {
    /// The window needs to be dragged.
    Window(window_event::Event),
    /// The theme has changed.
    ChangedThemeMode(Mode),
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
    /// Refresh the screen state.
    Refresh,
    /// The application theme configuration has changed.
    ChangeThemeConfig(config::application::Theme),
    /// The application language configuration has changed.
    ChangeLanguageConfig(config::application::Language),
}

/// Updates the state of the application.
pub fn update(state: &mut State, message: Message) -> Task<Message> {
    tracing::debug!(?message, "Processing message...");
    match message {
        Message::Window(event) => {
            window_event::update(&mut state.window_state, event).map(Message::Window)
        }
        Message::ChangedThemeMode(mode) => {
            if state.global_state.app_config.theme.is_system() {
                state.theme = mode.into();
            }

            Task::none()
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
        Message::Refresh => {
            state.screen.refresh();
            Task::none()
        }
        Message::ChangeThemeConfig(theme) => {
            state.global_state.app_config.theme = theme;

            let task = if !state.global_state.app_config.theme.is_system() {
                state.theme = state.global_state.app_config.theme.into();
                Task::none()
            } else {
                system::theme().map(Message::ChangedThemeMode)
            };

            let _ = state
                .global_state
                .app_config
                .save_to_project_dirs(&state.global_state.project_dirs);

            task
        }
        Message::ChangeLanguageConfig(language) => {
            state.global_state.app_config.language = language;
            let _ = change_language(&state.global_state.i18n, language);
            let _ = state
                .global_state
                .app_config
                .save_to_project_dirs(&state.global_state.project_dirs);
            Task::none()
        }
    }
}
