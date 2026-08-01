//! Contains utilities for working with I/O operations with platform and environment-specific optimizations, particularly for SSH subsystems on Unix-like systems.

use tokio::io::{AsyncRead, AsyncWrite, Result};

#[cfg(unix)]
mod unix;

/// Returns a platform and environment-specific implementation of an asynchronous reader for the standard input (stdin) of the current process.
///
/// # Safety
/// On Unix-like systems, this function is not safe to call when the standard input is not a pipe, as it will attempt to create a `File` from the raw file descriptor 0 (stdin) and insert it into a tokio's [`Receiver`]. If the standard input is not a pipe, this may lead to undefined behavior or runtime errors.
pub fn input() -> Result<impl AsyncRead> {
    #[cfg(unix)]
    {
        crate::io::unix::UnixInput::new()
    }

    #[cfg(not(unix))]
    {
        Ok(tokio::io::stdin())
    }
}

/// Returns a platform and environment-specific implementation of an asynchronous writer for the standard output (stdout) of the current process.
///
/// # Safety
/// On Unix-like systems, this function is not safe to call when the standard output is not a pipe, as it will attempt to create a `File` from the raw file descriptor 1 (stdout) and insert it into a tokio's [`Sender`]. If the standard output is not a pipe, this may lead to undefined behavior or runtime errors.
pub fn output() -> Result<impl AsyncWrite> {
    #[cfg(unix)]
    {
        crate::io::unix::UnixOutput::new()
    }

    #[cfg(not(unix))]
    {
        Ok(tokio::io::stdout())
    }
}
