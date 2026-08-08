//! This module contains the fonts for the Kitty Theme.

use std::borrow::Cow;

use iced_core::{
    Font,
    font::{Family, Stretch, Style, Weight},
};

/// Lato Regular font bytes
pub static LATO_REGULAR_BYTES: &[u8] = include_bytes!("./Lato-Regular.ttf");

/// Lato Bold font bytes
pub static LATO_BOLD_BYTES: &[u8] = include_bytes!("./Lato-Bold.ttf");

/// Constructs a Lato font with the given weight.
pub const fn lato_font(weight: Weight) -> Font {
    Font {
        weight,
        family: Family::Name("Lato"),
        stretch: Stretch::Normal,
        style: Style::Normal,
    }
}

/// Lato Regular font
pub const LATO_REGULAR_FONT: Font = lato_font(Weight::Normal);

/// Lato Bold font
pub const LATO_BOLD_FONT: Font = lato_font(Weight::Bold);

/// Material Symbols Rounded Filled font bytes
pub static MATERIAL_SYMBOLS_ROUNDED_FILLED_REGULAR_BYTES: &[u8] =
    include_bytes!("./MaterialSymbolsRounded_Filled-Regular.ttf");

/// Material Symbols Rounded Filled font
pub const MATERIAL_SYMBOLS_FILLED_ROUNDED_REGULAR_FONT: Font = Font {
    weight: Weight::Normal,
    family: Family::Name("Material Symbols Rounded Filled"),
    stretch: Stretch::Normal,
    style: Style::Normal,
};

/// A list of all the fonts from Kitty Theme.
pub static FONTS: &[&[u8]] = &[
    LATO_REGULAR_BYTES,
    LATO_BOLD_BYTES,
    MATERIAL_SYMBOLS_ROUNDED_FILLED_REGULAR_BYTES,
];

/// Returns a vector of all the fonts from Kitty Theme to be used in the application settings.
pub fn fonts() -> Vec<Cow<'static, [u8]>> {
    FONTS.iter().map(|&font| Cow::Borrowed(font)).collect()
}
