//! The dark palette for the Kitty Theme.

use iced_core::{
    color,
    theme::{
        Palette,
        palette::{Background, Danger, Extended, Pair, Primary, Secondary, Success, Warning},
    },
};

/// The palette for the dark theme.
pub const DARK_PALETTE: Palette = Palette {
    background: color!(0x000000),
    text: color!(0xffffff),
    primary: color!(0x8500ff),
    success: color!(0x00ff00),
    warning: color!(0xffff00),
    danger: color!(0xff0000),
};

/// The extended palette for the dark theme.
pub const DARK_EXTENDED_PALETTE: Extended = Extended {
    is_dark: true,
    background: Background {
        base: Pair {
            color: color!(0x000000),
            text: color!(0xffffff),
        },
        weakest: Pair {
            color: color!(0x0f0f0f),
            text: color!(0x515151),
        },
        weaker: Pair {
            color: color!(0x0f0f0f),
            text: color!(0x515151),
        },
        weak: Pair {
            color: color!(0x0f0f0f),
            text: color!(0x515151),
        },
        neutral: Pair {
            color: color!(0x232323),
            text: color!(0x515151),
        },
        strong: Pair {
            color: color!(0x232323),
            text: color!(0x515151),
        },
        stronger: Pair {
            color: color!(0x232323),
            text: color!(0x515151),
        },
        strongest: Pair {
            color: color!(0x232323),
            text: color!(0x515151),
        },
    },
    danger: Danger {
        base: Pair {
            color: color!(0xff0000),
            text: color!(0xffffff),
        },
        weak: Pair {
            color: color!(0xff0000),
            text: color!(0xffffff),
        },
        strong: Pair {
            color: color!(0xff0000),
            text: color!(0xffffff),
        },
    },
    primary: Primary {
        base: Pair {
            color: color!(0x8500ff),
            text: color!(0xffffff),
        },
        weak: Pair {
            color: color!(0x6a00ce),
            text: color!(0xffffff),
        },
        strong: Pair {
            color: color!(0x5301a0),
            text: color!(0xffffff),
        },
    },
    secondary: Secondary {
        base: Pair {
            color: color!(0x0085ff),
            text: color!(0xffffff),
        },
        weak: Pair {
            color: color!(0x0085ff),
            text: color!(0xffffff),
        },
        strong: Pair {
            color: color!(0x0085ff),
            text: color!(0xffffff),
        },
    },
    success: Success {
        base: Pair {
            color: color!(0x00ff00),
            text: color!(0xffffff),
        },
        weak: Pair {
            color: color!(0x00ff00),
            text: color!(0xffffff),
        },
        strong: Pair {
            color: color!(0x00ff00),
            text: color!(0xffffff),
        },
    },
    warning: Warning {
        base: Pair {
            color: color!(0xffff00),
            text: color!(0xffffff),
        },
        weak: Pair {
            color: color!(0xffff00),
            text: color!(0xffffff),
        },
        strong: Pair {
            color: color!(0xffff00),
            text: color!(0xffffff),
        },
    },
};
