//! The connection configuration module.

use std::{borrow::Cow, path::Path};

use directories::ProjectDirs;
use i18n_embed::unic_langid::LanguageIdentifier;
use kitty_theme_iced::theme::Theme;
use serde::{Deserialize, Serialize};

use crate::resources::ResourceLocation;

/// The theme of the application.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AppTheme {
    /// The theme is set to follow the system's theme.
    #[default]
    System,
    /// The theme is set to dark mode.
    Dark,
    /// The theme is set to light mode.
    Light,
}

impl AppTheme {
    /// Check if the theme is set to follow the system's theme.
    pub fn is_system(&self) -> bool {
        matches!(self, Self::System)
    }
}

impl From<AppTheme> for Theme {
    fn from(value: AppTheme) -> Self {
        match value {
            AppTheme::System => Self::default(),
            AppTheme::Dark => Self::Dark,
            AppTheme::Light => Self::Light,
        }
    }
}

/// The language of the application.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AppLanguage {
    /// The language is set to follow the system's language.
    #[default]
    System,
    /// The language is set to English.
    English,
    /// The language is set to Portuguese.
    Portuguese,
}

impl From<AppLanguage> for LanguageIdentifier {
    fn from(value: AppLanguage) -> Self {
        match value {
            AppLanguage::System => "en-US".parse().unwrap(),
            AppLanguage::English => "en-US".parse().unwrap(),
            AppLanguage::Portuguese => "pt-BR".parse().unwrap(),
        }
    }
}

/// The configuration of the application.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppConfig {
    /// The theme of the application.
    pub theme: AppTheme,
    /// The language of the application.
    pub language: AppLanguage,
    /// If the default SSH agent forwarding should be enable for new hosts.
    pub ssh_agent_forwarding: bool,
}

impl ResourceLocation for AppConfig {
    fn dir(project_dirs: &ProjectDirs) -> Cow<'_, Path> {
        project_dirs.config_local_dir().into()
    }

    fn filename() -> &'static str {
        "config.toml"
    }
}
