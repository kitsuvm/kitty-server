#![warn(missing_docs)]

//! Graphical user interface for the Kitty Server.

use iced::{
    Border, Color, Element, Length, Padding, Renderer, Subscription, Task,
    alignment::Vertical,
    border::Radius,
    color, exit, font,
    widget::{column, container, text},
    window::{
        Direction, Id, drag, drag_resize, is_maximized, latest, minimize, resize_events,
        toggle_maximize,
    },
};

use crate::{
    fonts::{LATO_REGULAR_BYTES, LATO_REGULAR_FONT, MATERIAL_SYMBOLS_FILLED_ROUNDED_BYTES},
    theme::Theme,
    widgets::{
        window::{Resize, window},
        window_bar::window_bar,
    },
};

mod fonts;
mod theme;
mod widgets;

/// The state of the application.
struct State {
    /// Whether the window is maximized.
    pub window_maximized: bool,
}

/// The messages of the application.
#[derive(Debug, Clone)]
enum Message {
    /// The window needs to be dragged.
    Drag,
    /// The window needs to be resized.
    DragResize(Direction),
    /// The window has been resized.
    ChangedResize(Id),
    /// The window has been maximized or unmaximized.
    ChangedMaximized(bool),
    /// The window needs to be minimized.
    Minimize,
    /// The window needs to toggle between maximized and unmaximized.
    Maximize,
    /// The window needs to be closed.
    Close,
}

/// The main function of the application.
fn main() -> iced::Result {
    iced::application::<State, Message, Theme, Renderer>(boot, update, view)
        .title("Kitty Server")
        .theme(Theme::Dark)
        .decorations(false)
        .transparent(true)
        .style(|_state: &State, theme: &Theme| iced::theme::Style {
            background_color: Color::TRANSPARENT,
            text_color: theme.palette().text,
        })
        .subscription(subscription)
        .run()
}

/// Boots the application.
fn boot() -> (State, Task<Message>) {
    (
        State {
            window_maximized: false,
        },
        Task::batch([
            font::load(LATO_REGULAR_BYTES)
                .map(|v| {
                    if let Err(e) = v {
                        eprintln!("Failed to load font: {:?}", e);
                    }
                })
                .discard(),
            font::load(MATERIAL_SYMBOLS_FILLED_ROUNDED_BYTES)
                .map(|v| {
                    if let Err(e) = v {
                        eprintln!("Failed to load font: {:?}", e);
                    }
                })
                .discard(),
        ]),
    )
}

/// Updates the state of the application.
fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::Drag => latest().then(|id| {
            if let Some(id) = id {
                drag(id)
            } else {
                Task::none()
            }
        }),
        Message::DragResize(direction) => latest().then(move |id| {
            if let Some(id) = id {
                drag_resize(id, direction)
            } else {
                Task::none()
            }
        }),
        Message::Close => exit(),
        Message::Minimize => latest().then(|id| {
            if let Some(id) = id {
                minimize(id, true)
            } else {
                Task::none()
            }
        }),
        Message::Maximize => latest().then(|id| {
            if let Some(id) = id {
                toggle_maximize(id)
            } else {
                Task::none()
            }
        }),
        Message::ChangedResize(id) => is_maximized(id).map(Message::ChangedMaximized),
        Message::ChangedMaximized(maximized) => {
            state.window_maximized = maximized;
            Task::none()
        }
    }
}

/// Renders the view of the application.
fn view(state: &State) -> Element<'_, Message, Theme, Renderer> {
    let is_maximized = state.window_maximized;

    window(
        if !state.window_maximized {
            Some(
                Resize::new()
                    .top(Message::DragResize(Direction::North))
                    .bottom(Message::DragResize(Direction::South))
                    .left(Message::DragResize(Direction::West))
                    .right(Message::DragResize(Direction::East))
                    .top_left(Message::DragResize(Direction::NorthWest))
                    .top_right(Message::DragResize(Direction::NorthEast))
                    .bottom_left(Message::DragResize(Direction::SouthWest))
                    .bottom_right(Message::DragResize(Direction::SouthEast)),
            )
        } else {
            None
        },
        container(column![
            window_bar(
                state.window_maximized,
                None::<Element<'_, Message, Theme, Renderer>>,
                Some(
                    text("Kitty Server")
                        .align_y(Vertical::Center)
                        .height(Length::Fill)
                        .font(LATO_REGULAR_FONT)
                )
            ),
            container(text("Hello Kitty!").font(LATO_REGULAR_FONT)).padding(Padding::from(5))
        ])
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |theme: &Theme| container::Style {
            border: if !is_maximized {
                Border {
                    radius: Radius::from(10.0),
                    color: match theme {
                        Theme::Light => color!(0xd1d1d1),
                        Theme::Dark => color!(0x2d2d2d),
                    },
                    width: 1.2,
                }
            } else {
                Border::default()
            },
            background: Some(theme.palette().background.into()),
            ..Default::default()
        })
        .padding(Padding::from(1)),
    )
}

/// Subscribes to window resize events.
fn subscription(_: &State) -> Subscription<Message> {
    resize_events().map(|(id, _)| Message::ChangedResize(id))
}
