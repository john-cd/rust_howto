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

/// Parses CLI arguments and prints a greeting.
///
/// # Errors
///
/// Returns an error when parsing fails or when the greeting action fails.
fn main() -> anyhow::Result<()> {
    println!("{}", greet(Args::parse())?);
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

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
