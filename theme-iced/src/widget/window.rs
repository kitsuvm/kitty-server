//! A window widget that can be used to create a custom window with a title bar and buttons.

use iced_core::{Element, Length, Pixels, theme::Base};
use iced_widget::{Row, button, column, container, space, stack};

use crate::{
    renderer::TextRenderer,
    widget::{icon, window_background, window_bar, window_button, window_resize},
    window_event,
};

#[derive(Debug, Clone, Default)]
/// A set of parameters for a [`Window`] widget.
pub struct Parameters {
    /// The buttons of the window bar.
    pub window_bar_buttons: Option<Vec<WindowButtons>>,
    /// Whether the buttons are on the left side of the window.
    pub window_bar_left_buttons: Option<bool>,
    /// Whether the window bar content is centered.
    pub window_bar_centered: bool,
    /// The width of the side content of the window bar.
    pub window_bar_side_width: Option<Length>,
    /// Whether the window bar buttons are animated.
    pub animated: bool,
    /// The animation mode of the window bar buttons.
    pub animation: Option<iced_anim::animated::Mode>,
    /// The size of the icons in the window bar buttons.
    pub icon_size: Option<Pixels>,
}

/// A catalog of styles for the window widget.
pub trait Catalog:
    window_background::Catalog
    + window_button::Catalog
    + iced_widget::text::Catalog
    + window_bar::Catalog
    + Base
{
    /// Returns the default parameters for the [`Window`] widget.
    fn default_parameters() -> Parameters;

    /// Converts a style function into a class for the window button.
    fn into_button_class<'a>(
        style: impl Fn(&Self, window_button::Status, button::Status) -> button::Style + 'a,
    ) -> <Self as window_button::Catalog>::SuperClass<'a>;

    /// Converts a style function into a class for the container.
    fn into_container_class<'a>(
        style: impl Fn(&Self) -> container::Style + 'a,
    ) -> <Self as container::Catalog>::Class<'a>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// An enum representing the buttons that can be displayed in a window.
pub enum WindowButtons {
    /// The minimize button.
    Minimize,
    /// The maximize button.
    Maximize,
    /// The close button.
    Close,
}

impl WindowButtons {
    /// Converts the [`WindowButtons`] enum into a [`window_button::WindowButton`] widget with the given message.
    pub fn into_button<'a, Message, Theme, Renderer>(
        self,
        message: Option<Message>,
        icon_size: Option<Pixels>,
        maximized: bool,
        animated: bool,
        animation: Option<iced_anim::animated::Mode>,
    ) -> window_button::WindowButton<'a, Message, Theme, Renderer>
    where
        Message: Clone + 'a,
        Theme: Catalog + 'a,
        Renderer: TextRenderer + 'a,
    {
        let mut icon = icon(match self {
            Self::Minimize => icon::MINIMIZE_ICON,
            Self::Maximize => match maximized {
                true => icon::UNMAXIMIZE_ICON,
                false => icon::MAXIMIZE_ICON,
            },
            Self::Close => icon::CLOSE_ICON,
        });

        if let Some(size) = icon_size {
            icon = icon.size(size);
        }

        let mut button = window_button(
            container(icon)
                .center(Length::Fill)
                .class(Theme::into_container_class(container::transparent)),
        )
        .no_rounded_corner(maximized)
        .animated(animated);

        if let Some(animation) = animation {
            button = button.animation(animation);
        }

        if self == Self::Close {
            button = button.class(Theme::into_button_class(window_button::danger))
        }

        match message {
            Some(msg) => button.on_press(msg),
            None => button,
        }
    }
}

impl From<WindowButtons> for window_event::Event {
    fn from(button: WindowButtons) -> Self {
        match button {
            WindowButtons::Minimize => window_event::Event::Minimize,
            WindowButtons::Maximize => window_event::Event::Maximize,
            WindowButtons::Close => window_event::Event::Close,
        }
    }
}

/// A window widget that can be used to create a custom window with a title bar and buttons.
pub struct Window<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: TextRenderer + 'a,
{
    /// The content of the window.
    content: Element<'a, Message, Theme, Renderer>,
    /// The state of the window.
    window_state: Option<window_event::State>,
    /// The function to call when the window is resized.
    on_event: Option<Box<dyn Fn(window_event::Event) -> Message + 'a>>,
    /// The centered content of the window bar.
    window_bar_center: Option<Element<'a, Message, Theme, Renderer>>,
    /// The buttons of the window bar.
    window_bar_buttons: Option<Vec<WindowButtons>>,
    /// The content for opposite side of the window buttons in the window bar.
    window_bar_opposite: Option<Element<'a, Message, Theme, Renderer>>,
    /// The extra content for the side of the window buttons.
    window_bar_extra: Option<Element<'a, Message, Theme, Renderer>>,
    /// Whether the buttons are on the left side of the window.
    window_bar_left_buttons: Option<bool>,
    /// Whether the window bar content is centered.
    window_bar_centered: bool,
    /// The width of the side content of the window bar.
    window_bar_side_width: Option<Length>,
    /// The size of the icons in the window bar buttons.
    icon_size: Option<Pixels>,
    /// Whether the window bar buttons are animated.
    animated: bool,
    /// The animation mode of the window bar buttons.
    animation: Option<iced_anim::animated::Mode>,
    /// The modal to be shown on top of the window content.
    modal: Option<Element<'a, Message, Theme, Renderer>>,
}

impl<'a, Message, Theme, Renderer> Window<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: TextRenderer + 'a,
{
    /// Creates a new [`Window`] widget with the given content.
    pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        let parameters = <Theme as Catalog>::default_parameters();

        Self {
            content: content.into(),
            window_state: None,
            on_event: None,
            window_bar_center: None,
            window_bar_opposite: None,
            window_bar_extra: None,
            modal: None,
            window_bar_buttons: parameters.window_bar_buttons,
            window_bar_left_buttons: parameters.window_bar_left_buttons,
            window_bar_centered: parameters.window_bar_centered,
            window_bar_side_width: parameters.window_bar_side_width,
            animation: parameters.animation,
            animated: parameters.animated,
            icon_size: parameters.icon_size,
        }
    }

    /// Sets the state of the window.
    pub fn window_state(mut self, state: window_event::State) -> Self {
        self.window_state = Some(state);
        self
    }

    /// Sets the function to call when the window is resized.
    pub fn on_event(mut self, f: impl Fn(window_event::Event) -> Message + 'a) -> Self {
        self.on_event = Some(Box::new(f));
        self
    }

    /// Sets the centered content of the window bar.
    pub fn window_bar_center(
        mut self,
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        self.window_bar_center = Some(content.into());
        self
    }

    /// Sets the buttons of the window bar.
    pub fn window_bar_buttons(mut self, buttons: impl IntoIterator<Item = WindowButtons>) -> Self {
        self.window_bar_buttons = Some(buttons.into_iter().collect());
        self
    }

    /// Sets the content for opposite side of the window buttons in the window bar.
    pub fn window_bar_opposite(
        mut self,
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        self.window_bar_opposite = Some(content.into());
        self
    }

    /// Sets the extra content for the side of the window buttons.
    pub fn window_bar_extra(
        mut self,
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        self.window_bar_extra = Some(content.into());
        self
    }

    /// Sets whether the buttons are on the left side of the window.
    pub fn window_bar_left_buttons(mut self, left_buttons: bool) -> Self {
        self.window_bar_left_buttons = Some(left_buttons);
        self
    }

    /// Sets whether the window bar content is centered.
    pub fn window_bar_centered(mut self, centered: bool) -> Self {
        self.window_bar_centered = centered;
        self
    }

    /// Sets the width of the side content of the window bar.
    pub fn window_bar_side_width(mut self, side_width: Length) -> Self {
        self.window_bar_side_width = Some(side_width);
        self
    }

    /// Sets whether the window bar buttons are animated.
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    /// Sets the size of the icons in the window bar buttons.
    pub fn icon_size(mut self, size: impl Into<Pixels>) -> Self {
        self.icon_size = Some(size.into());
        self
    }

    /// Sets the modal to be shown on top of the window content.
    pub fn modal(mut self, modal: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        self.modal = Some(modal.into());
        self
    }
}

impl<'a, Message, Theme, Renderer> From<Window<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: TextRenderer + 'a,
{
    fn from(window: Window<'a, Message, Theme, Renderer>) -> Self {
        let (raw_window_bar_buttons, left_buttons) = window.window_bar_buttons.map_or_else(
            || match window.window_bar_left_buttons {
                Some(true) => (
                    vec![
                        WindowButtons::Close,
                        WindowButtons::Minimize,
                        WindowButtons::Maximize,
                    ],
                    true,
                ),
                _ => (
                    vec![
                        WindowButtons::Minimize,
                        WindowButtons::Maximize,
                        WindowButtons::Close,
                    ],
                    false,
                ),
            },
            |v| (v, window.window_bar_left_buttons.unwrap_or_default()),
        );

        let last_window_bar_button_index = raw_window_bar_buttons.len().saturating_sub(1);

        let mut window_bar_buttons = raw_window_bar_buttons
            .into_iter()
            .enumerate()
            .map(|(i, raw_button)| {
                let message = window.on_event.as_ref().map(|f| (f)(raw_button.into()));

                Element::from(
                    raw_button
                        .into_button(
                            message,
                            window.icon_size,
                            window.window_state.is_some_and(|s| s.maximized),
                            window.animated,
                            window.animation,
                        )
                        .position(match i {
                            0 => window_button::Position::Left,
                            i if i == last_window_bar_button_index => {
                                window_button::Position::Right
                            }
                            _ => window_button::Position::Center,
                        }),
                )
            })
            .collect::<Vec<_>>();

        match (window.window_bar_extra, left_buttons) {
            (Some(content), false) => window_bar_buttons.insert(0, content),
            (Some(content), true) => window_bar_buttons.push(content),
            _ => {}
        };

        let mut window_bar = window_bar(window.window_bar_center.unwrap_or(space().into()))
            .buttons(Row::with_children(window_bar_buttons))
            .left_buttons(left_buttons)
            .centered(window.window_bar_centered);

        if let Some(side_width) = window.window_bar_side_width {
            window_bar = window_bar.side_width(side_width);
        }

        if let Some(ref on_event) = window.on_event {
            window_bar = window_bar
                .on_press(on_event(window_event::Event::Drag))
                .on_double_click(on_event(window_event::Event::Maximize));
        }

        if let Some(window_opposite) = window.window_bar_opposite {
            window_bar = window_bar.opposite(window_opposite);
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

        stack![window_content, window.modal, resize_area].into()
    }
}
