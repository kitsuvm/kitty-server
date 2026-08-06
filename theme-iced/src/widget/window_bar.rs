//! A window bar widget for the application.

use iced_core::{Element, Length, alignment::Horizontal, renderer};
use iced_widget::{container, mouse_area, row, space};

pub trait Catalog: container::Catalog {
    /// Converts a style function into a class for the window bar container.
    fn into_class<'a>(style: impl Fn(&Self) -> container::Style + 'a) -> Self::Class<'a>;
}

pub struct WindowBar<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: renderer::Renderer + 'a,
{
    /// The content of the window bar.
    content: Element<'a, Message, Theme, Renderer>,
    /// The buttons of the window bar.
    buttons: Option<Element<'a, Message, Theme, Renderer>>,
    /// The content of the window bar on the opposite side of the buttons.
    opposite: Option<Element<'a, Message, Theme, Renderer>>,
    /// Whether the window buttons are aligned to the left or right of the window.
    left_buttons: bool,
    /// The height of the window bar.
    height: Length,
    /// The message to send when the window bar is pressed.
    on_press: Option<Message>,
    /// The message to send when the window bar is double clicked.
    on_double_click: Option<Message>,
    /// Centers the content of the window bar if true, otherwise aligns it to the left.
    centered: bool,
    /// The width of the side content of the window bar.
    side_width: Option<Length>,
}

impl<'a, Message, Theme, Renderer> WindowBar<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: renderer::Renderer + 'a,
{
    /// Creates a new [`WindowBar`] widget with the given content.
    pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        Self {
            content: content.into(),
            buttons: None,
            opposite: None,
            left_buttons: false,
            height: 34.into(),
            on_press: None,
            on_double_click: None,
            centered: true,
            side_width: None,
        }
    }

    /// Sets the buttons of the window bar.
    pub fn buttons(mut self, buttons: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        self.buttons = Some(buttons.into());
        self
    }

    /// Sets the content of the window bar on the opposite side of the buttons.
    pub fn opposite(mut self, opposite: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        self.opposite = Some(opposite.into());
        self
    }

    /// Sets whether the window buttons are aligned to the left or right of the window.
    pub fn left_buttons(mut self, left_buttons: bool) -> Self {
        self.left_buttons = left_buttons;
        self
    }

    /// Sets the height of the window bar.
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the message to send when the window bar is pressed.
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Sets the message to send when the window bar is double clicked.
    pub fn on_double_click(mut self, message: Message) -> Self {
        self.on_double_click = Some(message);
        self
    }

    /// Sets the width of the side content of the window bar.
    pub fn centered(mut self, centered: bool) -> Self {
        self.centered = centered;
        self
    }

    /// Sets the width of the side content of the window bar.
    pub fn side_width(mut self, side_width: Length) -> Self {
        self.side_width = Some(side_width);
        self
    }
}

impl<'a, Message, Theme, Renderer> From<WindowBar<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: iced_core::renderer::Renderer + 'a,
{
    fn from(window_bar: WindowBar<'a, Message, Theme, Renderer>) -> Self {
        let opposite = window_bar.opposite.map(|c| {
            container(c)
                .center_y(window_bar.height)
                .class(Theme::into_class(container::transparent))
        });

        let buttons = window_bar.buttons.map(|c| {
            container(c)
                .center_y(window_bar.height)
                .class(Theme::into_class(container::transparent))
        });

        let mut center_slot = container(window_bar.content)
            .center_y(window_bar.height)
            .class(Theme::into_class(container::transparent));

        center_slot = match window_bar.centered {
            true => center_slot.align_x(Horizontal::Center),
            false => center_slot.align_left(Length::Fill),
        };

        let (mut left_slot, mut right_slot) = match window_bar.left_buttons {
            true => (buttons, opposite),
            false => (opposite, buttons),
        };

        if window_bar.centered {
            let side_width = window_bar.side_width.unwrap_or(Length::Fill);
            left_slot = left_slot.map(|c| c.align_left(side_width));
            right_slot = right_slot.map(|c| c.align_right(side_width));
        } else {
            right_slot = right_slot.map(|c| c.align_x(Horizontal::Right));
        }

        if left_slot.is_none() && window_bar.centered {
            left_slot =
                Some(container(space()).width(window_bar.side_width.unwrap_or(Length::Fill)));
        }

        if right_slot.is_none() && window_bar.centered {
            right_slot =
                Some(container(space()).width(window_bar.side_width.unwrap_or(Length::Fill)));
        }

        let bar = row![left_slot, center_slot, right_slot];

        if window_bar.on_press.is_some() || window_bar.on_double_click.is_some() {
            let mut interactive_bar = mouse_area(bar);

            if let Some(on_press) = window_bar.on_press {
                interactive_bar = interactive_bar.on_press(on_press);
            }

            if let Some(on_double_click) = window_bar.on_double_click {
                interactive_bar = interactive_bar.on_double_click(on_double_click);
            }

            interactive_bar.into()
        } else {
            bar.into()
        }
    }
}
