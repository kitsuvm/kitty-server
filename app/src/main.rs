//! Graphical user interface for the Kitty Server.

use iced::{Element, Renderer, Subscription, Task, window::Settings};
use kitty_theme_iced::{
    font::load_all,
    theme::Theme,
    widget::{animated::window, application::application_style, scaffold, text},
    window_event,
};

/// The state of the application.
struct State {
    /// Whether the window is maximized.
    pub window_state: window_event::State,
}

/// The messages of the application.
#[derive(Debug, Clone, Copy)]
enum Message {
    /// The window needs to be dragged.
    Window(window_event::Event),
}

/// The main function of the application.
fn main() -> iced::Result {
    iced::application::<State, Message, Theme, Renderer>(boot, update, view)
        .title("Kitty Server")
        .theme(|_: &State| Theme::Dark)
        .style(application_style)
        .subscription(subscription)
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
        },
        load_all(),
    )
}

/// Updates the state of the application.
fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::Window(event) => {
            window_event::update(&mut state.window_state, event).map(Message::Window)
        }
    }
}

/// Renders the view of the application.
fn view(state: &State) -> Element<'_, Message, Theme, Renderer> {
    window(
        scaffold(text("Hello Kitty!"))
            .sidebar(text("Sidebar").size(14).center())
            .bottom_sidebar(text("Sidebar").size(14).center()),
    )
    .on_event(Message::Window)
    .window_state(&state.window_state)
    .window_title(text("Kitty Server").size(14).center())
    .into()
}

/// Subscribes to window resize events.
fn subscription(_: &State) -> Subscription<Message> {
    window_event::subscription().map(Message::Window)
}
