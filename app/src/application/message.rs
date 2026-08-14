use iced::{Task, system, theme::Mode};
use kitty_theme_iced::window_event;

use crate::{
    application::{
        modal::{ModalKind, ModalState},
        screen::{Screen, ScreenState},
        state::{Lazy, State},
    },
    i18n::change_language,
    resources::{
        app_config::{AppConfig, AppLanguage, AppTheme},
        hosts::HostsManager,
    },
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
    /// The
    LoadedHostsManager(Lazy<HostsManager>),
    /// Refresh the screen state.
    Refresh,
    /// The application theme configuration has changed.
    ChangeAppTheme(AppTheme),
    /// The application language configuration has changed.
    ChangeAppLanguage(AppLanguage),
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
        Message::LoadedHostsManager(hosts_manager) => {
            match &mut state.screen {
                ScreenState::ServerList(state) => {
                    state.internal = hosts_manager;
                }
            }
            Task::none()
        }
        Message::Refresh => {
            state.screen.refresh();
            Task::none()
        }
        Message::ChangeAppTheme(theme) => {
            state.global_state.app_config.theme = theme;

            let task = if !state.global_state.app_config.theme.is_system() {
                state.theme = state.global_state.app_config.theme.into();
                Task::none()
            } else {
                system::theme().map(Message::ChangedThemeMode)
            };

            if let Err(e) = state
                .global_state
                .resource_manager
                .save(&state.global_state.app_config)
            {
                tracing::error!(?e, "Failed to save application configuration.");
            }

            task
        }
        Message::ChangeAppLanguage(language) => {
            let _ = change_language(&state.global_state.i18n, language);

            state.global_state.app_config.language = language;

            if let Err(e) = state
                .global_state
                .resource_manager
                .save(&state.global_state.app_config)
            {
                tracing::error!(?e, "Failed to save application configuration.");
            }
            Task::none()
        }
    }
}
