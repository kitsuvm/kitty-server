//! A text widget with the Lato Regular font.

use iced_core::Font;
pub use iced_widget::text::{Catalog, Style, Text};

use crate::font::LATO_REGULAR_FONT;

/// Returns a text widget with the Lato Regular font.
pub fn text<'a, Theme, Renderer>(
    text: impl iced_widget::text::IntoFragment<'a>,
) -> Text<'a, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: iced_core::text::Renderer,
    <Renderer as iced_core::text::Renderer>::Font: From<Font>,
{
    iced_widget::text(text).font(LATO_REGULAR_FONT)
}
