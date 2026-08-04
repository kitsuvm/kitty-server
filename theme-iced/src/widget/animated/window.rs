use iced_core::{Element, Length, theme::Base};
use iced_widget::{Row, button, column, container, space, stack};

pub use crate::widget::window::WindowButtons;
use crate::{
    widget::{animated::window_button, window_background, window_bar, window_resize},
    window_event,
};

/// A window widget that can be used to create a custom window with a title bar and buttons.
pub struct Window<'a, Message, Theme, Renderer> {
    /// The content of the window.
    content: Element<'a, Message, Theme, Renderer>,
    /// The title of the window.
    window_title: Option<Element<'a, Message, Theme, Renderer>>,
    /// The extra content of the window.
    window_extra: Option<Element<'a, Message, Theme, Renderer>>,
    /// The buttons of the window.
    window_buttons: Option<Vec<WindowButtons>>,
    /// The extra content for the side of the window buttons.
    window_extra_buttons: Option<Element<'a, Message, Theme, Renderer>>,
    /// The state of the window.
    window_state: Option<window_event::State>,
    /// The function to call when the window is resized.
    on_event: Option<Box<dyn Fn(window_event::Event) -> Message + 'a>>,
    /// Whether the buttons are on the left side of the window.
    left_buttons: bool,
}

impl<'a, Message, Theme, Renderer> Window<'a, Message, Theme, Renderer> {
    /// Creates a new [`Window`] widget with the given content.
    pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        Self {
            content: content.into(),
            window_title: None,
            window_extra: None,
            window_buttons: None,
            window_state: None,
            window_extra_buttons: None,
            on_event: None,
            left_buttons: false,
        }
    }

    /// Sets the title of the window.
    pub fn window_title(mut self, title: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        self.window_title = Some(title.into());
        self
    }

    /// Sets the extra content of the window.
    pub fn window_extra(mut self, extra: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        self.window_extra = Some(extra.into());
        self
    }

    /// Sets the buttons of the window.
    pub fn window_buttons(mut self, buttons: Vec<WindowButtons>) -> Self {
        self.window_buttons = Some(buttons);
        self
    }

    /// Sets the extra content for the side of the window buttons.
    pub fn window_extra_buttons(
        mut self,
        extra: impl Into<Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        self.window_extra_buttons = Some(extra.into());
        self
    }

    /// Sets the state of the window.
    pub fn window_state(mut self, state: &window_event::State) -> Self {
        self.window_state = Some(state.clone());
        self
    }

    /// Sets the function to call when the window is resized.
    pub fn on_event(mut self, f: impl Fn(window_event::Event) -> Message + 'a) -> Self {
        self.on_event = Some(Box::new(f));
        self
    }

    /// Sets whether the buttons are on the left side of the window.
    pub fn left_buttons(mut self, left: bool) -> Self {
        self.left_buttons = left;
        self
    }
}

impl<'a, Message, Theme, Renderer> From<Window<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: Base
        + window_background::Catalog
        + window_button::Catalog
        + iced_widget::button::Catalog
        + iced_widget::container::Catalog
        + iced_widget::text::Catalog
        + 'a,
    Renderer: iced_core::text::Renderer + 'a,
    <Renderer as iced_core::text::Renderer>::Font: From<iced_core::Font>,
    <Theme as window_background::Catalog>::Class<'a>: Into<window_background::StyleFn<'a, Theme>>,
    <Theme as window_button::Catalog>::Class<'a>:
        From<window_button::StyleFn<'a, Theme>> + Into<window_button::StyleFn<'a, Theme>>,
    <Theme as iced_widget::container::Catalog>::Class<'a>: From<container::StyleFn<'a, Theme>>,
    <Theme as iced_widget::button::Catalog>::Class<'a>:
        From<button::StyleFn<'a, Theme>> + Into<button::StyleFn<'a, Theme>>,
{
    fn from(window: Window<'a, Message, Theme, Renderer>) -> Self {
        let expected_window_buttons =
            window
                .window_buttons
                .unwrap_or_else(|| match window.left_buttons {
                    true => vec![
                        WindowButtons::Close,
                        WindowButtons::Minimize,
                        WindowButtons::Maximize,
                    ],
                    false => vec![
                        WindowButtons::Minimize,
                        WindowButtons::Maximize,
                        WindowButtons::Close,
                    ],
                });

        let last_window_button_index = expected_window_buttons.len().saturating_sub(1);

        let mut window_buttons = expected_window_buttons
            .into_iter()
            .enumerate()
            .map(|(i, button)| {
                Element::from(
                    match button {
                        WindowButtons::Minimize => button.into_animated_button(
                            window
                                .on_event
                                .as_ref()
                                .map(|f| (f)(window_event::Event::Minimize)),
                            window.window_state.map_or(false, |s| s.maximized),
                        ),
                        WindowButtons::Maximize => button.into_animated_button(
                            window
                                .on_event
                                .as_ref()
                                .map(|f| (f)(window_event::Event::Maximize)),
                            window.window_state.map_or(false, |s| s.maximized),
                        ),
                        WindowButtons::Close => button.into_animated_button(
                            window
                                .on_event
                                .as_ref()
                                .map(|f| (f)(window_event::Event::Close)),
                            window.window_state.map_or(false, |s| s.maximized),
                        ),
                    }
                    .no_rounded_corner(window.window_state.map_or(false, |s| s.maximized))
                    .left_buttons(window.left_buttons)
                    .position(match i {
                        0 => window_button::Position::Left,
                        i if i == last_window_button_index => window_button::Position::Right,
                        _ => window_button::Position::Center,
                    }),
                )
            })
            .collect::<Vec<_>>();

        match (window.window_extra_buttons, window.left_buttons) {
            (Some(extra), false) => window_buttons.insert(0, extra),
            (Some(extra), true) => window_buttons.push(extra),
            _ => {}
        };

        let mut window_bar = window_bar(window.window_title.unwrap_or(space().into()))
            .buttons(Row::with_children(window_buttons))
            .left_buttons(window.left_buttons);

        if let Some(ref on_event) = window.on_event {
            window_bar = window_bar
                .on_press(on_event(window_event::Event::Drag))
                .on_double_click(on_event(window_event::Event::Maximize));
        }

        if let Some(window_extra) = window.window_extra {
            window_bar = window_bar.extra(window_extra);
        }

        let mut window_content = window_background(column![window_bar, window.content]);

        if let Some(window_state) = window.window_state {
            window_content = window_content.status(window_state.as_ref().into());
        }

        let mut resize_area = window_resize(space().height(Length::Fill).width(Length::Fill));

        resize_area = match window.window_state {
            Some(window_state) => resize_area.handles(window_state.as_ref().into()),
            None => resize_area,
        };

        resize_area = match window.on_event {
            Some(on_event) => resize_area
                .on_resize(move |direction| on_event(window_event::Event::DragResize(direction))),
            None => resize_area,
        };

        stack![window_content, resize_area].into()
    }
}
