//! A text widget with the Lato Regular font.

use iced_core::Font;

use crate::font::LATO_REGULAR_FONT;

/// A custom renderer trait that natively bundles the font requirement
pub trait TextRenderer: iced_core::text::Renderer<Font: From<Font>> {}

// Blanket implementation so any renderer meeting the criteria automatically implements it
impl<T> TextRenderer for T where T: iced_core::text::Renderer<Font: From<Font>> {}

/// Returns a text widget with the Lato Regular font.
pub fn text<'a, Theme, Renderer>(
    text: impl iced_widget::text::IntoFragment<'a>,
) -> iced_widget::text::Text<'a, Theme, Renderer>
where
    Theme: iced_widget::text::Catalog + 'a,
    Renderer: TextRenderer,
{
    iced_widget::text(text).font(LATO_REGULAR_FONT)
}
