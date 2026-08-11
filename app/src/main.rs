//! Graphical user interface for the Kitty Server.

use iced::{Renderer, Subscription, Task};
use kitty_theme_iced::{
    theme::{Theme, default_settings, default_window_settings},
    widget::{application::application_style, window},
    window_event,
};

use crate::{
    modal::{ModalKind, ModalState, modal},
    screen::{Screen, ScreenState},
};

mod modal;
mod screen;

/// The state of the application.
#[derive(Debug, Clone, Default)]
struct State {
    /// Whether the window is maximized.
    pub window_state: window_event::State,
    /// The current screen of the application.
    pub screen: ScreenState,
    /// The current modal of the application.
    pub modal: ModalState,
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
        .window(default_window_settings())
        .run()
}

/// Boots the application.
fn boot() -> (State, Task<Message>) {
    (State::default(), Task::none())
}

/// Updates the state of the application.
fn update(state: &mut State, message: Message) -> Task<Message> {
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
            state.modal = ModalState::None;
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

    if let Some(modal_content) = modal(&state.modal) {
        window = window.modal(modal_content);
    }

    window
}

/// Subscribes to window resize events.
fn subscription(_: &State) -> Subscription<Message> {
    window_event::subscription().map(Message::Window)
}
