//! A widget for displaying icons using the Material Symbols Filled Rounded Regular font.

use iced_core::{Font, Pixels, text::IntoFragment};
pub use iced_widget::text::{Catalog, Style, Text};
use iced_widget::text_input;

use crate::{font::MATERIAL_SYMBOLS_FILLED_ROUNDED_REGULAR_FONT, widget::text::TextRenderer};

/// Returns a text widget with the Material Symbols Filled Rounded Regular font.
pub fn icon<'a, Theme, Renderer>(text: impl IntoFragment<'a>) -> Text<'a, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: TextRenderer,
{
    iced_widget::text(text).font(MATERIAL_SYMBOLS_FILLED_ROUNDED_REGULAR_FONT)
}

/// Returns a text input icon with the Material Symbols Filled Rounded Regular font.
pub fn to_text_input_icon(
    code_point: char,
    spacing: f32,
    size: Option<Pixels>,
) -> text_input::Icon<Font> {
    text_input::Icon {
        font: MATERIAL_SYMBOLS_FILLED_ROUNDED_REGULAR_FONT,
        code_point,
        size,
        spacing,
        side: text_input::Side::Right,
    }
}

/// Minimize icon using Material Symbols.
pub const MINIMIZE_ICON: char = '\u{e931}';
/// Maximize icon using Material Symbols.
pub const MAXIMIZE_ICON: char = '\u{f830}';
/// Unmaximize icon using Material Symbols.
pub const UNMAXIMIZE_ICON: char = '\u{f507}';
/// Close icon using Material Symbols.
pub const CLOSE_ICON: char = '\u{e5cd}';
/// Add icon using Material Symbols.
pub const ADD_ICON: char = '\u{e145}';
/// Search icon using Material Symbols.
pub const SEARCH_ICON: char = '\u{e8b6}';
