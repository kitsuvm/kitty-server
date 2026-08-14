use clap::{Parser, ValueEnum};

use crate::resources::app_config::{AppLanguage, AppTheme};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, ValueEnum)]
pub enum CliTheme {
    /// Use the system theme.
    #[default]
    System,
    /// Use the dark theme.
    Dark,
    /// Use the light theme.
    Light,
}

impl From<CliTheme> for AppTheme {
    fn from(value: CliTheme) -> Self {
        match value {
            CliTheme::System => Self::System,
            CliTheme::Dark => Self::Dark,
            CliTheme::Light => Self::Light,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, ValueEnum)]
pub enum CliLanguage {
    /// Use the system language.
    #[default]
    System,
    /// Use English.
    English,
    /// Use Portuguese.
    Portuguese,
}

impl From<CliLanguage> for AppLanguage {
    fn from(value: CliLanguage) -> Self {
        match value {
            CliLanguage::System => Self::System,
            CliLanguage::English => Self::English,
            CliLanguage::Portuguese => Self::Portuguese,
        }
    }
}

#[derive(Parser)]
pub struct Cli {
    /// The theme of the application.
    #[arg(short, long)]
    pub theme: Option<CliTheme>,
    /// The language of the application.
    #[arg(short, long)]
    pub language: Option<CliLanguage>,
    /// The server to connect to.
    pub server: Option<String>,
}
