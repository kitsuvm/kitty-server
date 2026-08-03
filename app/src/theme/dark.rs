//! Dark mode for the Kitty Theme for Iced.

use iced::{color, theme::Palette};

/// The palette for the dark theme.
pub const PALETTE: Palette = Palette {
    background: color!(0x000000),
    text: color!(0xffffff),
    primary: color!(0x8500ff),
    success: color!(0x00ff00),
    warning: color!(0xffff00),
    danger: color!(0xff0000),
};
