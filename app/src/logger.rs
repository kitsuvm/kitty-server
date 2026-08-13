//! Logger module for the Kitty Server application.

use std::fs;

use directories::ProjectDirs;
use tracing_appender::{non_blocking::WorkerGuard, rolling};
use tracing_subscriber::{EnvFilter, Layer, layer::SubscriberExt, util::SubscriberInitExt};

use crate::Error;

/// Logger struct that holds the project directories and the worker guard for the non-blocking log appender.
pub struct Logger {
    /// The project directories for the application.
    pub project_dirs: ProjectDirs,
    /// The worker guard for the non-blocking log appender.
    _worker_guard: WorkerGuard,
}

/// Initializes the logger for the Kitty Server application.
pub fn init() -> Result<Logger, Error> {
    let project_dirs = ProjectDirs::from("com", "KitsuVM", "Kitty Server").ok_or_else(|| {
        eprintln!("Could not determine project directories.");
        Error::ProjectDirs
    })?;

    let log_dir = project_dirs.data_local_dir().join("logs");

    fs::create_dir_all(&log_dir).map_err(|e| {
        eprintln!("Could not create log directory: {}", e);
        Error::CreateLogDir
    })?;

    let log_file = rolling::Builder::new()
        .filename_suffix("log")
        .max_log_files(3)
        .latest_symlink("latest.log")
        .rotation(rolling::Rotation::MINUTELY)
        .build(log_dir)
        .map_err(|e| {
            eprintln!("Could not create log file: {}", e);
            Error::CreateLogFile
        })?;

    let (non_blocking, _worker_guard) = tracing_appender::non_blocking(log_file);

    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_filter(
            tracing_subscriber::filter::Targets::new()
                .with_target("kitty_server_app", tracing::Level::TRACE),
        );

    let stdout_layer = tracing_subscriber::fmt::layer().with_filter(
        EnvFilter::try_from_env("KITTY_SERVER_LOG")
            .unwrap_or_else(|_| EnvFilter::new("kitty_server_app=info")),
    );

    tracing_subscriber::registry()
        .with(file_layer)
        .with(stdout_layer)
        .try_init()
        .map(|_| Logger {
            project_dirs,
            _worker_guard,
        })
        .map_err(|e| {
            eprintln!("Could not initialize tracing subscriber: {}", e);
            Error::TracingInit
        })
}
