//! Utilities for working with SSH on Unix-like systems.

use std::{fs::File, os::fd::FromRawFd};

use tokio::{
    io,
    net::unix::pipe::{Receiver, Sender},
};

/// Returns a [`Receiver`] that reads from the standard input (stdin) of the current process using a Unix pipe, allowing for asynchronous reading of data.
///
/// # Safety
/// This function isn't safe to call when the standard input is not a pipe, as it will attempt to create a `File` from the raw file descriptor 0 (stdin) and insert it into a tokio's [`Receiver`]. If the standard input is not a pipe, this may lead to undefined behavior or runtime errors.
pub fn input() -> io::Result<Receiver> {
    let stdin = unsafe { File::from_raw_fd(0) };

    Receiver::from_file(stdin)
}

/// Returns a [`Sender`] that writes to the standard output (stdout) of the current process using a Unix pipe, allowing for asynchronous writing of data.
///
/// # Safety
/// This function isn't safe to call when the standard output is not a pipe, as it will attempt to create a `File` from the raw file descriptor 1 (stdout) and insert it into a tokio's [`Sender`]. If the standard output is not a pipe, this may lead to undefined behavior or runtime errors.
pub fn output() -> io::Result<Sender> {
    let stdout = unsafe { File::from_raw_fd(1) };

    Sender::from_file(stdout)
}
