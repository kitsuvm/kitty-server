//! This module contains the main application logic for the Kitty Server application, including initialization, event handling, and rendering of the user interface.

use iced::{Length, Padding, Renderer, Subscription, system, widget::container};
use kitty_theme_iced::{
    theme::{Theme, default_settings, default_window_settings},
    widget::{application::application_style, icon, icon_button, window},
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
    i18n::I18n,
    resources::{
        ResourceManager,
        app_config::{AppConfig, AppTheme},
    },
};

pub mod message;
pub mod modal;
pub mod screen;
pub mod state;

/// Initializes the application and runs the main event loop.
pub fn init(
    resource_manager: ResourceManager,
    i18n: I18n,
    app_config: AppConfig,
    theme: AppTheme,
) -> Result<(), Error> {
    iced::application::<State, Message, Theme, Renderer>(
        boot(resource_manager, i18n, app_config, theme),
        update,
        view,
    )
    .title("Kitty Server")
    .theme(|state: &State| state.theme)
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
    Subscription::batch([
        window_event::subscription().map(Message::Window),
        system::theme_changes().map(Message::ChangedThemeMode),
    ])
}

/// Renders the view of the application based on the current state.
fn view<'a>(state: &'a State) -> window::Window<'a, Message, Theme, Renderer> {
    tracing::trace!("Rendering view...");
    let mut window = window(state.screen.content(&state.global_state))
        .on_event(Message::Window)
        .window_state(state.window_state)
        .window_bar_extra(
            container(
                icon_button(icon::MENU_ICON)
                    .on_press(Message::OpenModal(modal::ModalKind::Configuration)),
            )
            .padding(Padding::from(0).right(10))
            .align_bottom(Length::Fill),
        );

    if let Some(opposite) = state.screen.window_bar_opposite() {
        window = window.window_bar_opposite(opposite);
    }

    if let Some(center) = state.screen.window_bar_center(&state.global_state) {
        window = window.window_bar_center(center);
    }

    if let Some(side_width) = state.screen.window_bar_side_width() {
        window = window.window_bar_side_width(side_width)
    }

    if let Some(modal_content) = modal(&state.modal, &state.global_state) {
        window = window.modal(modal_content);
    }

    window
}
