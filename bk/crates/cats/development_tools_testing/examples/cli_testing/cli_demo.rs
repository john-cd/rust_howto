// ANCHOR: example
//! A small executable used by the command-line integration tests.

use clap::Parser;

/// Arguments accepted by the test executable.
#[derive(Debug, Parser)]
#[command(name = "cli-demo")]
struct Args {
    /// Name to greet.
    #[arg(long, default_value = "world")]
    name: String,
    /// Return an error to demonstrate a failing command.
    #[arg(long)]
    fail: bool,
}

/// Prints a greeting or returns the requested demonstration error.
///
/// # Errors
///
/// Returns an error when the fail option is selected.
/// Parses CLI arguments and runs the greeting example.
///
/// # Errors
///
/// Returns an error when `--fail` is supplied.
fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    if args.fail {
        anyhow::bail!("requested failure");
    }
    println!("Hello, {}!", args.name);
    Ok(())
}
// ANCHOR_END: example
