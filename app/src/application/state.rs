use std::fmt;

use iced::{Task, application::BootFn, system};
use kitty_theme_iced::{theme::Theme, window_event};

use crate::{
    application::{message::Message, modal::ModalState, screen::ScreenState},
    i18n::I18n,
    resources::{
        ResourceManager,
        app_config::{AppConfig, AppTheme},
        hosts::HostsManager,
    },
};

/// Represents the state of a lazy-loaded resource, which can be in one of three states: loading, data, or error.
#[derive(Default)]
pub enum Lazy<T> {
    /// The resource is currently loading.
    #[default]
    Loading,
    /// The resource has been successfully loaded and contains data.
    Data(T),
    /// An error occurred while loading the resource, with an associated error message.
    Error(String),
}

impl<T> From<Result<T, String>> for Lazy<T> {
    fn from(result: Result<T, String>) -> Self {
        match result {
            Ok(data) => Self::Data(data),
            Err(err) => Self::Error(err),
        }
    }
}

impl<T> From<T> for Lazy<T> {
    fn from(data: T) -> Self {
        Self::Data(data)
    }
}

impl<T: fmt::Debug> fmt::Debug for Lazy<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Loading => write!(f, "LazyState::Loading"),
            Self::Data(data) => write!(f, "LazyState::Data({:?})", data),
            Self::Error(err) => write!(f, "LazyState::Error({:?})", err),
        }
    }
}

impl<T: Clone> Clone for Lazy<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Loading => Self::Loading,
            Self::Data(data) => Self::Data(data.clone()),
            Self::Error(err) => Self::Error(err.clone()),
        }
    }
}

/// The global state of the application.
#[derive(Debug)]
pub struct GlobalState {
    /// The project directories of the application.
    pub resource_manager: ResourceManager,
    /// The i18n language loader of the application.
    pub i18n: I18n,
    /// The configuration of the application.
    pub app_config: AppConfig,
}

/// The state of the application.
#[derive(Debug)]
pub struct State {
    /// Whether the window is maximized.
    pub window_state: window_event::State,
    /// The current screen of the application.
    pub screen: ScreenState,
    /// The current modal of the application.
    pub modal: ModalState,
    /// The global state of the application.
    pub global_state: GlobalState,
    /// The theme of the application.
    pub theme: Theme,
}

/// Boots the application, loading the servers from the configuration file.
pub fn boot(
    resource_manager: ResourceManager,
    i18n: I18n,
    app_config: AppConfig,
    theme: AppTheme,
) -> impl BootFn<State, Message> {
    tracing::info!("Booting application...");

    move || {
        let resource_manager = resource_manager.clone();

        (
            State {
                window_state: window_event::State::default(),
                screen: ScreenState::default(),
                modal: ModalState::None,
                global_state: GlobalState {
                    resource_manager: resource_manager.clone(),
                    i18n: i18n.clone(),
                    app_config: app_config.clone(),
                },
                theme: theme.into(),
            },
            Task::batch([
                Task::perform(async move { HostsManager::new(&resource_manager) }, |v| {
                    Message::LoadedHostsManager(v.into())
                }),
                system::theme().map(Message::ChangedThemeMode),
            ]),
        )
    }
}

#[macro_export]
/// A macro for translating messages using the i18n system.
macro_rules! t {
    // 1. Basic translation without arguments: t!(state, "message-id")
    ($state:expr, $message_id:literal) => {
        i18n_embed_fl::fl!($state.i18n.borrow(), $message_id)
    };

    // 2. Translation with arguments (supports named `key = val`, positional `val`, or expressions)
    ($state:expr, $message_id:literal, $($args:tt)*) => {
        i18n_embed_fl::fl!($state.i18n.borrow(), $message_id, $($args)*)
    };
}
