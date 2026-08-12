//! Graphical user interface for the Kitty Server.

use std::{fs, process::ExitCode};

use directories::ProjectDirs;
use iced::{Renderer, Subscription, Task, application::BootFn};
use kitty_theme_iced::{
    theme::{Theme, default_settings, default_window_settings},
    widget::{application::application_style, window},
    window_event,
};
use tracing_subscriber::{EnvFilter, Layer, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    modal::{ModalKind, ModalState, modal},
    screen::{Screen, ScreenState},
    server::{Servers, ServersState},
};

mod modal;
mod screen;
mod server;

/// The global state of the application.
#[derive(Debug, Clone)]
struct GlobalState {
    /// The project directories of the application.
    pub project_dirs: ProjectDirs,
}

/// The state of the application.
#[derive(Debug, Clone)]
struct State {
    /// Whether the window is maximized.
    pub window_state: window_event::State,
    /// The current screen of the application.
    pub screen: ScreenState,
    /// The current modal of the application.
    pub modal: ModalState,
    /// The global state of the application.
    pub global_state: GlobalState,
}

/// The messages of the application.
#[derive(Debug, Clone)]
enum Message {
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

/// The main function of the application.
fn main() -> ExitCode {
    let Some(dirs) = ProjectDirs::from("com", "KitsuVM", "Kitty Server") else {
        eprintln!("Could not determine project directories.");
        return ExitCode::from(1);
    };

    let log_dir = dirs.data_local_dir().join("logs");
    if let Err(e) = fs::create_dir_all(&log_dir) {
        eprintln!("Could not create log directory: {}", e);
        return ExitCode::from(2);
    }

    let log_file = match tracing_appender::rolling::Builder::new()
        .filename_suffix("log")
        .max_log_files(3)
        .latest_symlink("latest.log")
        .rotation(tracing_appender::rolling::Rotation::MINUTELY)
        .build(log_dir)
    {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Could not create log file: {}", e);
            return ExitCode::from(3);
        }
    };

    let (non_blocking, _guard) = tracing_appender::non_blocking(log_file);

    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_filter(
            tracing_subscriber::filter::Targets::new()
                .with_target("kitty_server_app", tracing::Level::TRACE),
        );

    let stdout_layer = tracing_subscriber::fmt::layer().with_filter(
        EnvFilter::try_from_env("KITTY_SERVER_LOG")
            .unwrap_or_else(|_| EnvFilter::new("kitty_server_app=info")),
    );

    if let Err(e) = tracing_subscriber::registry()
        .with(file_layer)
        .with(stdout_layer)
        .try_init()
    {
        eprintln!("Could not initialize tracing subscriber: {}", e);
        return ExitCode::from(4);
    }

    if let Err(e) = iced::application::<State, Message, Theme, Renderer>(boot(dirs), update, view)
        .title("Kitty Server")
        .theme(|_: &State| Theme::Dark)
        .style(application_style)
        .subscription(subscription)
        .settings(iced::Settings {
            id: Some("kitty-server".into()),
            ..default_settings()
        })
        .window(default_window_settings())
        .run()
    {
        tracing::error!("Application error: {}", e);
        ExitCode::from(5)
    } else {
        ExitCode::SUCCESS
    }
}

fn boot(project_dirs: ProjectDirs) -> impl BootFn<State, Message> {
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

/// Updates the state of the application.
fn update(state: &mut State, message: Message) -> Task<Message> {
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
                    let mut servers = Servers::load_from_file(Servers::file_path(
                        &state.global_state.project_dirs,
                    ))
                    .unwrap_or_default();

                    servers.ssh_servers.push(modal.as_ref().into());
                    if let Err(e) =
                        servers.save_to_file(Servers::file_path(&state.global_state.project_dirs))
                    {
                        tracing::error!("Could not save connection configuration file: {}", e);
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

/// Renders the view of the application.
fn view<'a>(state: &'a State) -> window::Window<'a, Message, Theme, Renderer> {
    tracing::trace!(?state, "Rendering view...");
    let mut window = window(state.screen.content(&state.global_state))
        .on_event(Message::Window)
        .window_state(state.window_state);

    if let Some(opposite) = state.screen.window_bar_opposite() {
        window = window.window_bar_opposite(opposite);
    }

    if let Some(center) = state.screen.window_bar_center() {
        window = window.window_bar_center(center);
    }

    if let Some(side_width) = state.screen.window_bar_side_width() {
        window = window.window_bar_side_width(side_width)
    }

    if let Some(modal_content) = modal(&state.modal) {
        window = window.modal(modal_content);
    }

    window
}

/// Subscribes to window resize events.
fn subscription(_: &State) -> Subscription<Message> {
    window_event::subscription().map(Message::Window)
}
