//! Utilities for working with SSH on Windows.

use tokio::io;

/// Returns an [`io::Stdin`] that reads from the standard input (stdin) of the current process using Tokio's wrapper.
pub fn input() -> io::Result<io::Stdin> {
    Ok(io::stdin())
}

/// Returns an [`io::Stdout`] that writes to the standard output (stdout) of the current process using Tokio's wrapper.
pub fn output() -> io::Result<io::Stdout> {
    Ok(io::stdout())
}
