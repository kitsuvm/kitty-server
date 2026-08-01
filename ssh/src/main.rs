//! This is the main entry point for the SSH subsystem program.

use std::process::ExitCode;

use tokio::runtime::Builder;

mod io;

/// The program's entry point.
fn main() -> ExitCode {
    let runtime = match Builder::new_multi_thread().enable_all().build() {
        Ok(rt) => rt,
        Err(e) => {
            eprintln!("Failed to create Tokio runtime: {}", e);
            return ExitCode::from(1);
        }
    };

    runtime.block_on(async_main())
}

async fn async_main() -> ExitCode {
    let mut input = match io::input() {
        Ok(input) => input,
        Err(e) => {
            eprintln!("Failed to get stdin: {}", e);
            return ExitCode::from(2);
        }
    };

    let mut output = match io::output() {
        Ok(output) => output,
        Err(e) => {
            eprintln!("Failed to get stdout: {}", e);
            return ExitCode::from(3);
        }
    };

    ExitCode::SUCCESS
}
