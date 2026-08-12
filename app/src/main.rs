//! Graphical user interface for the Kitty Server.

use std::process::{ExitCode, Termination};

mod application;
mod config;
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
}

impl Termination for Error {
    fn report(self) -> ExitCode {
        ExitCode::from(self as u8)
    }
}

/// The main function of the application.
fn main() -> Result<(), Error> {
    let logger = logger::init()?;
    application::init(logger.project_dirs)
}
