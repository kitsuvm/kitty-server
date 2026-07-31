//! Contains utilities for working as a SSH subsystem.

use std::io::{IsTerminal, stdin};

use tokio::io::{self, AsyncRead};

#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;

/// Returns true if the current process is running in a SSH session by checking the presence of the `SSH_CONNECTION` environment variable.
pub fn is_ssh() -> bool {
    std::env::var("SSH_CONNECTION").is_ok()
}

/// Returns true if the current process has the standard input (stdin) attached to a terminal (TTY).
pub fn is_tty() -> bool {
    stdin().is_terminal()
}

/// Returns true if the current process is running as a SSH subsystem.
///
/// It checks if the process is running in a SSH session and not attached to a terminal (TTY).
pub fn is_ssh_subsystem() -> bool {
    is_ssh() && !is_tty()
}

/// Returns an OS-specific implementation of an asynchronous reader for the standard input (stdin) of the current process.
///
/// # Safety
/// On Unix-like systems, this function is not safe to call when the standard input is not a pipe, as it will attempt to create a `File` from the raw file descriptor 0 (stdin) and insert it into a tokio's [`Receiver`]. If the standard input is not a pipe, this may lead to undefined behavior or runtime errors.
pub fn input() -> io::Result<impl AsyncRead> {
    #[cfg(unix)]
    {
        unix::input()
    }

    #[cfg(windows)]
    {
        windows::input()
    }
}

/// Returns an OS-specific implementation of an asynchronous writer for the standard output (stdout) of the current process.
///
/// # Safety
/// On Unix-like systems, this function is not safe to call when the standard output is not a pipe, as it will attempt to create a `File` from the raw file descriptor 1 (stdout) and insert it into a tokio's [`Sender`]. If the standard output is not a pipe, this may lead to undefined behavior or runtime errors.
pub fn output() -> io::Result<impl tokio::io::AsyncWrite> {
    #[cfg(unix)]
    {
        unix::output()
    }

    #[cfg(windows)]
    {
        windows::output()
    }
}
