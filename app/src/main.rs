//! Graphical user interface for the Kitty Server.

use std::process::{ExitCode, Termination};

use clap::Parser;

use crate::resources::{
    ResourceManager,
    app_config::{AppConfig, AppLanguage},
};

mod application;
mod cli;
mod i18n;
mod logger;
mod resources;

/// The error codes for the application.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// The application failed to initialize the project directories.
    ProjectDirs = 1,
    /// The application failed to create the log directory.
    CreateLogDir = 2,
    /// The application failed to create the log file.
    CreateLogFile = 3,
    /// The application failed to initialize tracing.
    TracingInit = 4,
    /// The application failed to initialize the application.
    ApplicationInit = 5,
    /// The application failed to initialize the i18n system.
    I18nInit = 6,
    /// The application failed to load the resource manager.
    ResourceManagerLoad = 7,
}

impl Termination for Error {
    fn report(self) -> ExitCode {
        ExitCode::from(self as u8)
    }
}

/// The main function of the application.
fn main() -> Result<(), Error> {
    let logger = logger::init()?;

    let cli = cli::Cli::parse();

    let resource_manager = ResourceManager::new(logger.project_dirs);

    let app_config = resource_manager.load::<AppConfig>().map_err(|e| {
        tracing::error!(?e, "Failed to load application configuration.");
        Error::ResourceManagerLoad
    })?;

    let i18n = i18n::init(cli.language.map_or(app_config.language, AppLanguage::from))?;

    let theme = cli.theme.map_or(app_config.theme, |t| t.into());

    application::init(resource_manager, i18n, app_config, theme)
}
