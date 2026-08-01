//! Utilities for working with SSH on Unix-like systems.

use std::{
    fs::File,
    io::{IsTerminal, Result},
    os::fd::FromRawFd,
    pin::Pin,
    task::{Context, Poll},
};

use tokio::{
    io::{self, AsyncRead, AsyncWrite, ReadBuf, Stdin, stdin},
    net::unix::pipe::{Receiver, Sender},
};

/// Returns true if the current process is running in a SSH session by checking the presence of the `SSH_CONNECTION` environment variable.
fn is_ssh() -> bool {
    std::env::var("SSH_CONNECTION").is_ok()
}

/// Returns true if the current process has the standard input (stdin) attached to a terminal (TTY).
fn is_tty() -> bool {
    std::io::stdin().is_terminal()
}

/// Returns true if the current process is running as a SSH subsystem.
///
/// It checks if the process is running in a SSH session and not attached to a terminal (TTY).
fn is_ssh_subsystem() -> bool {
    is_ssh() && !is_tty()
}

/// An enum representing the different types of asynchronous input sources available on Unix-like systems.
pub enum UnixInput {
    /// A Unix pipe input source, represented by a [`Receiver`], generally used for use in SSH subsystems where the standard input is a pipe.
    Pipe(Receiver),
    // A standard input source, represented by a [`Stdin`].
    Tokio(Stdin),
}

impl UnixInput {
    /// Creates a new [`UnixInput`] instance that reads from the standard input (stdin) of the current process.
    ///
    /// If the process is running as a SSH subsystem, it will create a [`UnixInput::Pipe`] variant using a Unix pipe. Otherwise, it will create a [`UnixInput::Tokio`] variant using the standard input.
    pub fn new() -> io::Result<Self> {
        if is_ssh_subsystem() {
            let stdin = unsafe { File::from_raw_fd(0) };

            let receiver = Receiver::from_file(stdin)?;

            Ok(UnixInput::Pipe(receiver))
        } else {
            Ok(UnixInput::Tokio(stdin()))
        }
    }
}

impl AsyncRead for UnixInput {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<Result<()>> {
        match self.get_mut() {
            UnixInput::Pipe(receiver) => Pin::new(receiver).poll_read(cx, buf),
            UnixInput::Tokio(stdin) => Pin::new(stdin).poll_read(cx, buf),
        }
    }
}

/// An enum representing the different types of asynchronous output sources available on Unix-like systems.
pub enum UnixOutput {
    /// A Unix pipe output source, represented by a [`Sender`], generally used for use in SSH subsystems where the standard output is a pipe.
    Pipe(Sender),
    // A standard output source, represented by a [`tokio::io::Stdout`].
    Tokio(tokio::io::Stdout),
}

impl UnixOutput {
    /// Creates a new [`UnixOutput`] instance that writes to the standard output (stdout) of the current process.
    ///
    /// If the process is running as a SSH subsystem, it will create a [`UnixOutput::Pipe`] variant using a Unix pipe. Otherwise, it will create a [`UnixOutput::Tokio`] variant using the standard output.
    pub fn new() -> io::Result<Self> {
        if is_ssh_subsystem() {
            let stdout = unsafe { File::from_raw_fd(1) };

            let sender = Sender::from_file(stdout)?;

            Ok(UnixOutput::Pipe(sender))
        } else {
            Ok(UnixOutput::Tokio(tokio::io::stdout()))
        }
    }
}

impl AsyncWrite for UnixOutput {
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<Result<usize>> {
        match self.get_mut() {
            UnixOutput::Pipe(sender) => Pin::new(sender).poll_write(cx, buf),
            UnixOutput::Tokio(stdout) => Pin::new(stdout).poll_write(cx, buf),
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        match self.get_mut() {
            UnixOutput::Pipe(sender) => Pin::new(sender).poll_flush(cx),
            UnixOutput::Tokio(stdout) => Pin::new(stdout).poll_flush(cx),
        }
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        match self.get_mut() {
            UnixOutput::Pipe(sender) => Pin::new(sender).poll_shutdown(cx),
            UnixOutput::Tokio(stdout) => Pin::new(stdout).poll_shutdown(cx),
        }
    }
}
