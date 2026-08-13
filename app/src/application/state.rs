use directories::ProjectDirs;
use iced::{Task, application::BootFn, system};
use kitty_theme_iced::{theme::Theme, window_event};

use crate::{
    application::{message::Message, modal::ModalState, screen::ScreenState},
    config::{application::ApplicationConfig, servers::Servers},
    i18n::I18n,
};

/// The global state of the application.
#[derive(Debug, Clone)]
pub struct GlobalState {
    /// The project directories of the application.
    pub project_dirs: ProjectDirs,
    /// The i18n language loader of the application.
    pub i18n: I18n,
    /// The application configuration.
    pub app_config: ApplicationConfig,
}

/// The state of the application.
#[derive(Debug, Clone)]
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
    project_dirs: ProjectDirs,
    i18n: I18n,
    app_config: ApplicationConfig,
) -> impl BootFn<State, Message> {
    tracing::info!("Booting application...");

    move || {
        let project_dirs = project_dirs.clone();
        let theme = app_config.theme.into();

        (
            State {
                window_state: window_event::State::default(),
                screen: ScreenState::default(),
                modal: ModalState::None,
                global_state: GlobalState {
                    project_dirs: project_dirs.clone(),
                    i18n: i18n.clone(),
                    app_config: app_config.clone(),
                },
                theme,
            },
            Task::batch([
                Task::perform(
                    async move { Servers::load_from_project_dirs(&project_dirs) },
                    |v| Message::ServersUpdate(v.into()),
                ),
                system::theme().map(Message::ChangedThemeMode),
            ]),
        )
    }
}

#[macro_export]
/// A macro for translating messages using the i18n system.
macro_rules! t {
    // 1. Basic translation without arguments: t!(state, "message-id")
    ($state:expr, $message_id:expr) => {
        i18n_embed_fl::fl!($state.i18n.borrow(), $message_id)
    };

    // 2. Translation with arguments (supports named `key = val`, positional `val`, or expressions)
    ($state:expr, $message_id:expr, $($args:tt)*) => {
        i18n_embed_fl::fl!($state.i18n.borrow(), $message_id, $($args)*)
    };
}
