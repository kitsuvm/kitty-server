//! Graphical user interface for the Kitty Server.

use iced::{
    Padding, Renderer, Subscription, Task,
    widget::{column, container, row},
};
use kitty_theme_iced::{
    font::load_all,
    theme::Theme,
    widget::{
        application::application_style, icon, text, window_background, window_bar, window_button,
        window_resize,
    },
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
        .theme(Theme::Dark)
        .decorations(false)
        .transparent(true)
        .style(application_style)
        .subscription(subscription)
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
fn view(state: &State) -> window_resize::WindowResize<'_, Message, Theme, Renderer> {
    window_resize(
        window_background(column![
            window_bar(text("Kitty Server").size(14).center())
                .buttons(row![
                    window_button(icon(icon::MINIMIZE_ICON))
                        .position(window_button::Position::Left)
                        .on_press(Message::Window(window_event::Event::Minimize)),
                    window_button(icon(match state.window_state.maximized {
                        true => icon::UNMAXIMIZE_ICON,
                        false => icon::MAXIMIZE_ICON,
                    }))
                    .position(window_button::Position::Center)
                    .on_press(Message::Window(window_event::Event::Maximize)),
                    window_button(icon(icon::CLOSE_ICON))
                        .position(window_button::Position::Right)
                        .on_press(Message::Window(window_event::Event::Close))
                        .style(window_button::danger)
                        .no_rounded_corner(state.window_state.maximized),
                ])
                .on_press(Message::Window(window_event::Event::Drag))
                .on_double_click(Message::Window(window_event::Event::Maximize)),
            container(text("Hello Kitty!")).padding(Padding::from(5))
        ])
        .status(state.window_state.as_ref().into()),
    )
    .handles(state.window_state.as_ref().into())
    .on_resize(|direction| Message::Window(window_event::Event::DragResize(direction)))
}

/// Subscribes to window resize events.
fn subscription(_: &State) -> Subscription<Message> {
    window_event::subscription().map(Message::Window)
}
