//! Graphical user interface for the Kitty Server.

use std::borrow::Cow;

use iced::{Renderer, Subscription, Task, window::Settings};
use kitty_theme_iced::{
    theme::{Theme, default_settings},
    widget::{application::application_style, window},
    window_event,
};

use crate::screen::{Screen, ScreenState, ScreenType};

mod screen;

/// The state of the application.
struct State {
    /// Whether the window is maximized.
    pub window_state: window_event::State,
    /// The current screen of the application.
    pub screen: ScreenState,
}

/// The messages of the application.
#[derive(Debug, Clone)]
enum Message {
    /// The window needs to be dragged.
    Window(window_event::Event),
    /// The search input has changed.
    SearchInputChanged(String),
    /// The screen needs to be changed.
    ChangeScreen(ScreenType),
}

/// The main function of the application.
fn main() -> iced::Result {
    iced::application::<State, Message, Theme, Renderer>(boot, update, view)
        .title("Kitty Server")
        .theme(|_: &State| Theme::Dark)
        .style(application_style)
        .subscription(subscription)
        .settings(iced::Settings {
            id: Some("kitty-server".into()),
            ..default_settings()
        })
        .window(Settings {
            min_size: Some((400, 300).into()),
            decorations: false,
            transparent: true,
            ..Default::default()
        })
        .run()
}

/// Boots the application.
fn boot() -> (State, Task<Message>) {
    (
        State {
            window_state: Default::default(),
            screen: ScreenState::new(),
        },
        Task::none(),
    )
}

/// Updates the state of the application.
fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::Window(event) => {
            window_event::update(&mut state.window_state, event).map(Message::Window)
        }
        Message::SearchInputChanged(query) => {
            state.screen.set_search_query(query);
            Task::none()
        }
        Message::ChangeScreen(screen_type) => {
            state.screen = screen_type.into();
            Task::none()
        }
    }
}

/// Renders the view of the application.
fn view<'a>(state: &'a State) -> window::Window<'a, Message, Theme, Renderer> {
    let mut window = window(state.screen.content())
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

    window
}

/// Subscribes to window resize events.
fn subscription(_: &State) -> Subscription<Message> {
    window_event::subscription().map(Message::Window)
}
