use directories::ProjectDirs;
use iced::{Task, application::BootFn};
use kitty_theme_iced::window_event;

use crate::{
    application::{message::Message, modal::ModalState, screen::ScreenState},
    config::servers::Servers,
};

/// The global state of the application.
#[derive(Debug, Clone)]
pub struct GlobalState {
    /// The project directories of the application.
    pub project_dirs: ProjectDirs,
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
}

/// Boots the application, loading the servers from the configuration file.
pub fn boot(project_dirs: ProjectDirs) -> impl BootFn<State, Message> {
    tracing::info!("Booting application...");
    move || {
        let async_project_dirs = project_dirs.clone();

        (
            State {
                window_state: window_event::State::default(),
                screen: ScreenState::default(),
                modal: ModalState::None,
                global_state: GlobalState {
                    project_dirs: project_dirs.clone(),
                },
            },
            Task::perform(
                async move { Servers::load_from_file(Servers::file_path(&async_project_dirs)) },
                |v| Message::ServersUpdate(v.into()),
            ),
        )
    }
}
