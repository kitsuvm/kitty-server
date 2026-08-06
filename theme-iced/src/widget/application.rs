//! This module contains the style for the application background.

use iced_core::theme::Style;

/// A trait for defining the style of the entire application background.
pub trait Catalog {
    /// Returns the style for the application background.
    fn style(&self) -> Style;
}

/// Returns the style for the application background.
pub fn application_style<State, Theme>(_: &State, theme: &Theme) -> Style
where
    Theme: Catalog,
{
    theme.style()
}
