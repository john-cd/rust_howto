//! Runs one of the `url` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod base;
mod fragment;
mod new;
mod origin;
mod origin1;
mod parse;

/// Command-line arguments.
#[derive(Parser)]
struct Cli {
    /// The example to run.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// The examples that can be run.
#[derive(Subcommand)]
enum Commands {
    /// Runs the `base` example.
    #[command(name = "base")]
    Base,
    /// Runs the `fragment` example.
    #[command(name = "fragment")]
    Fragment,
    /// Runs the `new` example.
    #[command(name = "new")]
    New,
    /// Runs the `origin` example.
    #[command(name = "origin")]
    Origin,
    /// Runs the `origin1` example.
    #[command(name = "origin1")]
    Origin1,
    /// Runs the `parse` example.
    #[command(name = "parse")]
    Parse,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Base => base::run()?,
            Commands::Fragment => fragment::run()?,
            Commands::New => new::run()?,
            Commands::Origin => origin::run().map_err(anyhow::Error::from)?,
            Commands::Origin1 => origin1::run()?,
            Commands::Parse => parse::run()?,
        }
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_cli_dispatcher() {
        let _ = Cli::try_parse_from(["example"]);
    }
}
