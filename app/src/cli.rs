use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, ValueEnum)]
pub enum Theme {
    /// Use the system theme.
    #[default]
    System,
    /// Use the dark theme.
    Dark,
    /// Use the light theme.
    Light,
}

impl From<Theme> for crate::config::application::Theme {
    fn from(value: Theme) -> Self {
        match value {
            Theme::System => Self::System,
            Theme::Dark => Self::Dark,
            Theme::Light => Self::Light,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, ValueEnum)]
pub enum Language {
    /// Use the system language.
    #[default]
    System,
    /// Use English.
    English,
    /// Use Portuguese.
    Portuguese,
}

impl From<Language> for crate::config::application::Language {
    fn from(value: Language) -> Self {
        match value {
            Language::System => Self::System,
            Language::English => Self::English,
            Language::Portuguese => Self::Portuguese,
        }
    }
}

#[derive(Parser)]
pub struct Cli {
    /// The theme of the application.
    #[arg(short, long)]
    pub theme: Option<Theme>,
    /// The language of the application.
    #[arg(short, long)]
    pub language: Option<Language>,
    /// The server to connect to.
    pub server: Option<String>,
}
