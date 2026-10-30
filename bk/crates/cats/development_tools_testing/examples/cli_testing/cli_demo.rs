// ANCHOR: example
//! A small executable used by the command-line integration tests.

use std::ffi::OsString;

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

/// Parses CLI arguments and prints a greeting.
///
/// # Errors
///
/// Returns an error when parsing fails or when the greeting action fails.
fn main() -> anyhow::Result<()> {
    run_from(std::env::args_os())
}

/// Parses `args` and prints the resulting greeting.
///
/// Help, version, and usage errors are reported by clap, which exits with the
/// conventional status (0 for `--help`, 2 for invalid usage).
///
/// # Errors
///
/// Returns an error if greeting creation fails.
fn run_from<I, S>(args: I) -> anyhow::Result<()>
where
    I: IntoIterator<Item = S>,
    S: Into<OsString> + Clone,
{
    println!(
        "{}",
        greet(Args::try_parse_from(args).unwrap_or_else(|e| e.exit()))?
    );
    Ok(())
}

/// Builds the greeting or returns the requested demonstration error.
///
/// # Errors
///
/// Returns an error when `--fail` is supplied.
fn greet(args: Args) -> anyhow::Result<String> {
    if args.fail {
        anyhow::bail!("requested failure");
    }
    Ok(format!("Hello, {}!", args.name))
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies parsed arguments produce the expected greeting.
    #[test]
    fn test_cli_demo_greets_the_requested_name() -> anyhow::Result<()> {
        let args = Args::try_parse_from(["cli-demo", "--name", "Ada"])?;
        assert_eq!(greet(args)?, "Hello, Ada!");
        Ok(())
    }

    /// Verifies the example reports its requested failure case.
    #[test]
    fn test_cli_demo_returns_requested_failure() -> anyhow::Result<()> {
        let args = Args::try_parse_from(["cli-demo", "--fail"])?;
        assert!(greet(args).is_err());
        Ok(())
    }

    /// Runs the entry-point argument path with deterministic arguments.
    #[test]
    fn test_main_entry_point() -> anyhow::Result<()> {
        run_from(["cli-demo"])?;
        Ok(())
    }
}
