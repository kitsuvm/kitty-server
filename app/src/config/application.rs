//! The connection configuration module.

use std::{
    fmt, fs,
    path::{Path, PathBuf},
};

use directories::ProjectDirs;
use i18n_embed::unic_langid::LanguageIdentifier;
use kitty_theme_iced::theme;
use serde::{Deserialize, Serialize};

/// The theme of the application.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Theme {
    /// The theme is set to follow the system's theme.
    #[default]
    System,
    /// The theme is set to dark mode.
    Dark,
    /// The theme is set to light mode.
    Light,
}

impl Theme {
    /// Check if the theme is set to follow the system's theme.
    pub fn is_system(&self) -> bool {
        matches!(self, Self::System)
    }
}

impl From<Theme> for theme::Theme {
    fn from(value: Theme) -> Self {
        match value {
            Theme::System => Self::default(),
            Theme::Dark => Self::Dark,
            Theme::Light => Self::Light,
        }
    }
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Theme::System => write!(f, "system"),
            Theme::Dark => write!(f, "dark"),
            Theme::Light => write!(f, "light"),
        }
    }
}

/// The language of the application.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    /// The language is set to follow the system's language.
    #[default]
    System,
    /// The language is set to English.
    English,
    /// The language is set to Portuguese.
    Portuguese,
}

impl From<Language> for LanguageIdentifier {
    fn from(value: Language) -> Self {
        match value {
            Language::System => "en-US".parse().unwrap(),
            Language::English => "en-US".parse().unwrap(),
            Language::Portuguese => "pt-BR".parse().unwrap(),
        }
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Language::System => write!(f, "system"),
            Language::English => write!(f, "english"),
            Language::Portuguese => write!(f, "portuguese"),
        }
    }
}

/// The configuration of the application.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApplicationConfig {
    /// The theme of the application.
    pub theme: Theme,
    /// The language of the application.
    pub language: Language,
}

impl ApplicationConfig {
    /// Override the configuration with the command line arguments.
    pub fn override_with_cli(mut self, cli: &crate::cli::Cli) -> Self {
        if let Some(theme) = cli.theme {
            self.theme = theme.into();
        }

        if let Some(language) = cli.language {
            self.language = language.into();
        }

        self
    }

    /// Get the file path of the application configuration file.
    pub fn file_path(project_dirs: &ProjectDirs) -> PathBuf {
        project_dirs.config_dir().join("config.toml")
    }

    /// Load the application configuration from a file. If the file does not exist, create a new one with default values.
    pub fn load_from_file(path: impl AsRef<Path>) -> Self {
        let exists = match fs::exists(&path) {
            Ok(v) => v,
            Err(e) => {
                tracing::error!(
                    "Could not check if connection configuration file exists: {}",
                    e
                );
                return Self::default();
            }
        };

        if !exists {
            tracing::warn!("Connection configuration file does not exist, creating a new one.");

            let defaults = Self::default();

            if let Err(e) = defaults.save_to_file(&path) {
                tracing::error!("Could not create connection configuration file: {}", e);
                return defaults;
            }
        }

        let data = match fs::read(path) {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("Could not read connection configuration file: {}", e);
                return Self::default();
            }
        };

        match toml::from_slice::<Self>(&data) {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("Could not parse connection configuration file: {}", e);
                Self::default()
            }
        }
    }

    /// Load the application configuration from the project directories.
    pub fn load_from_project_dirs(project_dirs: &ProjectDirs) -> Self {
        Self::load_from_file(Self::file_path(project_dirs))
    }

    /// Save the application configuration to a file.
    pub fn save_to_file(&self, path: impl AsRef<Path>) -> Result<(), String> {
        fs::create_dir_all(path.as_ref().parent().ok_or_else(|| {
            tracing::error!("Could not get parent directory of connection configuration file.");
            "Could not get parent directory of connection configuration file.".to_string()
        })?)
        .map_err(|e| {
            tracing::error!("Could not create connection configuration directory: {}", e);
            e.to_string()
        })?;

        let data = toml::to_string(self).map_err(|e| {
            tracing::error!("Could not serialize connection configuration: {}", e);
            e.to_string()
        })?;

        fs::write(path, data).map_err(|e| {
            tracing::error!("Could not write connection configuration file: {}", e);
            e.to_string()
        })
    }

    /// Save the application configuration to the project directories.
    pub fn save_to_project_dirs(&self, project_dirs: &ProjectDirs) -> Result<(), String> {
        self.save_to_file(Self::file_path(project_dirs))
    }
}
