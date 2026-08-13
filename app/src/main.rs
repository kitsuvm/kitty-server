//! Graphical user interface for the Kitty Server.

use std::process::{ExitCode, Termination};

use clap::Parser;

use crate::config::application::ApplicationConfig;

mod application;
mod cli;
mod config;
mod i18n;
mod logger;

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
    let app_config =
        ApplicationConfig::load_from_project_dirs(&logger.project_dirs).override_with_cli(&cli);

    let i18n = i18n::init(app_config.language)?;

    application::init(logger.project_dirs, i18n, app_config)
}
