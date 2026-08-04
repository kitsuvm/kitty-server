//! A widget for displaying icons using the Material Symbols Filled Rounded Regular font.

pub use iced::widget::text::{Catalog, Style, Text};
use iced::{Font, widget};
use iced_core::text::IntoFragment;

use crate::font::MATERIAL_SYMBOLS_FILLED_ROUNDED_REGULAR_FONT;

/// Returns a text widget with the Material Symbols Filled Rounded Regular font.
pub fn icon<'a, Theme, Renderer>(text: impl IntoFragment<'a>) -> Text<'a, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: iced_core::text::Renderer,
    <Renderer as iced_core::text::Renderer>::Font: From<Font>,
{
    widget::text(text).font(MATERIAL_SYMBOLS_FILLED_ROUNDED_REGULAR_FONT)
}

/// Minimize icon using Material Symbols.
pub const MINIMIZE_ICON: char = '\u{e931}';
/// Maximize icon using Material Symbols.
pub const MAXIMIZE_ICON: char = '\u{f830}';
/// Unmaximize icon using Material Symbols.
pub const UNMAXIMIZE_ICON: char = '\u{f507}';
/// Close icon using Material Symbols.
pub const CLOSE_ICON: char = '\u{e5cd}';
