//! A window bar widget for the application.

use iced_core::{Element, Length};
use iced_widget::{container, mouse_area, row, space};

pub struct WindowBar<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: 'a,
    Renderer: iced_core::renderer::Renderer + 'a,
{
    /// The content of the window bar.
    content: Element<'a, Message, Theme, Renderer>,
    /// The buttons of the window bar.
    buttons: Option<Element<'a, Message, Theme, Renderer>>,
    /// The extra content of the window bar.
    extra: Option<Element<'a, Message, Theme, Renderer>>,
    /// Whether the window buttons are aligned to the left or right of the window.
    left_buttons: bool,
    /// The height of the window bar.
    height: Length,
    /// The message to send when the window bar is pressed.
    on_press: Option<Message>,
    /// The message to send when the window bar is double clicked.
    on_double_click: Option<Message>,
}

impl<'a, Message, Theme, Renderer> WindowBar<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: 'a,
    Renderer: iced_core::renderer::Renderer + 'a,
{
    /// Creates a new [`WindowBar`] widget with the given content.
    pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        Self {
            content: content.into(),
            buttons: None,
            extra: None,
            left_buttons: false,
            height: 30.into(),
            on_press: None,
            on_double_click: None,
        }
    }

    /// Sets the buttons of the window bar.
    pub fn buttons(mut self, buttons: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        self.buttons = Some(buttons.into());
        self
    }

    /// Sets the extra content of the window bar.
    pub fn extra(mut self, extra: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        self.extra = Some(extra.into());
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
}

impl<'a, Message, Theme, Renderer> From<WindowBar<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: container::Catalog + 'a,
    Renderer: iced_core::renderer::Renderer + 'a,
    <Theme as container::Catalog>::Class<'a>: From<container::StyleFn<'a, Theme>>,
{
    fn from(window_bar: WindowBar<'a, Message, Theme, Renderer>) -> Self {
        let extra = container(
            window_bar
                .extra
                .map_or_else(|| Element::from(space()), From::from),
        )
        .center_y(window_bar.height)
        .style(container::transparent);

        let buttons = container(
            window_bar
                .buttons
                .map_or(Element::from(space()), From::from),
        )
        .center_y(window_bar.height)
        .style(container::transparent);

        let center_slot = container(window_bar.content)
            .center_x(Length::Shrink)
            .center_y(window_bar.height)
            .style(container::transparent);

        let (left_slot, right_slot) = if window_bar.left_buttons {
            (buttons, extra)
        } else {
            (extra, buttons)
        };

        let bar = mouse_area(row![
            left_slot.align_left(Length::Fill),
            center_slot,
            right_slot.align_right(Length::Fill),
        ]);

        match (window_bar.on_press, window_bar.on_double_click) {
            (Some(on_press), Some(on_double_click)) => bar
                .on_press(on_press)
                .on_double_click(on_double_click)
                .into(),
            (Some(on_press), None) => bar.on_press(on_press).into(),
            (None, Some(on_double_click)) => bar.on_double_click(on_double_click).into(),
            (None, None) => bar.into(),
        }
    }
}
