//! This module re-exports the `iced_widget::text` module, which provides types and functions for working with text in Iced.
pub use iced_widget::text::{
    Alignment, Catalog, Format, Fragment, Highlighter, IntoFragment, LineHeight, Rich, Shaping,
    Span, State, Style, StyleFn, Text, Wrapping,
};

use crate::BaseExtended;

/// Creates a new [`Style`] with the danger color from the theme.
pub fn danger<Theme>(theme: &Theme) -> Style
where
    Theme: Catalog + BaseExtended,
{
    Style {
        color: Some(theme.palette_extended().danger.base.color),
    }
}
