//! This module provides a simple interface for handling window events in an Iced application.

use iced::{
    Subscription, Task, exit,
    window::{
        Direction, Id, drag, drag_resize, is_maximized, latest, minimize, resize_events,
        toggle_maximize,
    },
};

use crate::widget::{window, window_resize};

/// Represents the state of a window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct State {
    /// Whether the window is maximized.
    pub maximized: bool,
}

impl AsRef<State> for State {
    fn as_ref(&self) -> &State {
        self
    }
}

impl From<&State> for window::Status {
    fn from(state: &State) -> Self {
        if state.maximized {
            window::Status::Maximized
        } else {
            window::Status::Normal
        }
    }
}

impl From<&State> for window_resize::Handles {
    fn from(state: &State) -> Self {
        if state.maximized {
            window_resize::Handles::Disabled
        } else {
            window_resize::Handles::Clickable
        }
    }
}

/// Represents the events that can occur in a window.
#[derive(Debug, Clone, Copy)]
pub enum Event {
    /// The window needs to be dragged.
    Drag,
    /// The window needs to be resized.
    DragResize(Direction),
    /// The window has been resized.
    ChangedResize(Id),
    /// The window has been maximized or unmaximized.
    ChangedMaximize(bool),
    /// The window needs to be minimized.
    Minimize,
    /// The window needs to toggle between maximized and unmaximized.
    Maximize,
    /// The window needs to be closed.
    Close,
}

/// Subscribes to window events.
pub fn subscription() -> Subscription<Event> {
    resize_events().map(|(id, _)| Event::ChangedResize(id))
}

/// Updates the state of the window based on the given event.
pub fn update(state: &mut State, event: Event) -> Task<Event> {
    match event {
        Event::Drag => latest().then(|id| {
            if let Some(id) = id {
                drag(id)
            } else {
                Task::none()
            }
        }),
        Event::DragResize(direction) => latest().then(move |id| {
            if let Some(id) = id {
                drag_resize(id, direction)
            } else {
                Task::none()
            }
        }),
        Event::Close => exit(),
        Event::Minimize => latest().then(|id| {
            if let Some(id) = id {
                minimize(id, true)
            } else {
                Task::none()
            }
        }),
        Event::Maximize => latest().then(|id| {
            if let Some(id) = id {
                toggle_maximize(id)
            } else {
                Task::none()
            }
        }),
        Event::ChangedResize(id) => is_maximized(id).map(Event::ChangedMaximize),
        Event::ChangedMaximize(maximize) => {
            state.maximized = maximize;
            Task::none()
        }
    }
}
