use iced_core::{Element, color, theme::Base};
use iced_widget::{Button, button};

/// The position of the window button.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Position {
    /// The button is aligned to the left.
    Left,
    /// The button is aligned to the center.
    Center,
    /// The button is aligned to the right.
    Right,
}

/// The status of the window button.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Status {
    /// The status of the button.
    pub button_status: button::Status,
    /// The position of the button.
    pub button_position: Position,
    /// If the window buttons are aligned to the left or right of the window.
    pub left_buttons: bool,
    /// If the button should have no rounded corners.
    pub no_rounded_corner: bool,
}

/// A catalog of styles for window buttons.
pub trait Catalog {
    /// The class of the window button.
    type Class<'a>;

    /// Returns the default style of the window button.
    fn default<'a>() -> Self::Class<'a>;

    /// Returns the style of the window button based on its class and status.
    fn style<'a>(&self, class: &Self::Class<'_>, status: Status) -> button::Style;
}

/// A type alias for a style function that takes a theme and a status and returns a button style.
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> button::Style + 'a>;

/// A window button widget to create new buttons.
pub struct WindowButton<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: Catalog + button::Catalog + 'a,
    Renderer: iced_core::renderer::Renderer + 'a,
{
    /// The button widget.
    button_content: Button<'a, Message, Theme, Renderer>,
    /// The position of the button.
    position: Position,
    /// If the window buttons are aligned to the left or right of the window.
    left_buttons: bool,
    /// The style function for the button.
    class: <Theme as Catalog>::Class<'a>,
    /// If the button should have no rounded corners.
    no_rounded_corner: bool,
}

impl<'a, Message, Theme, Renderer> WindowButton<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: Catalog + button::Catalog + 'a,
    Renderer: iced_core::renderer::Renderer + 'a,
{
    /// Creates a new [`WindowButton`] widget with the given content.
    pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self
    where
        <Theme as Catalog>::Class<'a>: Into<StyleFn<'a, Theme>>,
    {
        Self {
            button_content: button(content),
            position: Position::Center,
            left_buttons: false,
            class: <Theme as Catalog>::default(),
            no_rounded_corner: false,
        }
    }

    /// Sets the message to send when the button is pressed.
    pub fn on_press(mut self, message: Message) -> Self {
        self.button_content = self.button_content.on_press(message);
        self
    }

    /// Sets the position of the button.
    pub fn position(mut self, position: Position) -> Self {
        self.position = position;
        self
    }

    /// Sets whether the window buttons are aligned to the left or right of the window.
    pub fn left_buttons(mut self, left_buttons: bool) -> Self {
        self.left_buttons = left_buttons;
        self
    }

    /// Sets the style function for the button.
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> button::Style + 'a) -> Self
    where
        <Theme as Catalog>::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
        self
    }

    /// Sets the class of the button.
    pub fn class(mut self, class: impl Into<<Theme as Catalog>::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }

    /// Sets whether the button should have no rounded corners.
    pub fn no_rounded_corner(mut self, no_rounded_corner: bool) -> Self {
        self.no_rounded_corner = no_rounded_corner;
        self
    }
}

impl<'a, Message, Theme, Renderer> From<WindowButton<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: Catalog + button::Catalog + 'a,
    Renderer: iced_core::renderer::Renderer + 'a,
    <Theme as iced_widget::button::Catalog>::Class<'a>:
        From<iced_widget::button::StyleFn<'a, Theme>>,
{
    fn from(window_button: WindowButton<'a, Message, Theme, Renderer>) -> Self {
        window_button
            .button_content
            .style(move |theme: &Theme, status: button::Status| {
                <Theme as Catalog>::style(
                    theme,
                    &window_button.class,
                    Status {
                        button_status: status,
                        button_position: window_button.position,
                        left_buttons: window_button.left_buttons,
                        no_rounded_corner: window_button.no_rounded_corner,
                    },
                )
            })
            .into()
    }
}

pub fn danger<'a, Theme>(theme: &Theme, status: Status) -> button::Style
where
    Theme: Catalog + Base + button::Catalog + 'a,
{
    let base = <Theme as Catalog>::style(theme, &<Theme as Catalog>::default(), status);

    if matches!(
        status.button_status,
        button::Status::Pressed | button::Status::Hovered
    ) {
        base.with_background(theme.palette().map_or(color!(0xff000), |p| p.danger))
    } else {
        base
    }
}
