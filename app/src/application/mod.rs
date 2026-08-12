//! This module contains the main application logic for the Kitty Server application, including initialization, event handling, and rendering of the user interface.

use directories::ProjectDirs;
use iced::{Renderer, Subscription};
use kitty_theme_iced::{
    theme::{Theme, default_settings, default_window_settings},
    widget::{application::application_style, window},
    window_event,
};

use crate::{
    Error,
    application::{
        message::{Message, update},
        modal::modal,
        screen::Screen,
        state::{State, boot},
    },
};

pub mod message;
pub mod modal;
pub mod screen;
pub mod state;

/// Initializes the application and runs the main event loop.
pub fn init(project_dirs: ProjectDirs) -> Result<(), Error> {
    iced::application::<State, Message, Theme, Renderer>(boot(project_dirs), update, view)
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
        .map_err(|e| {
            tracing::error!(?e, "Application failed to run.");
            Error::ApplicationInit
        })
}

/// Subscribes to window resize events.
fn subscription(_: &State) -> Subscription<Message> {
    window_event::subscription().map(Message::Window)
}

/// Renders the view of the application based on the current state.
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
