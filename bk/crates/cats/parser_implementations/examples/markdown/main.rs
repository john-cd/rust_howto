//! Runs one of the `markdown` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod comrak;
mod markdown;
mod pulldown_cmark;

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
    /// Runs the `comrak` example.
    #[command(name = "comrak")]
    Comrak,
    /// Runs the `markdown` example.
    #[command(name = "markdown")]
    Markdown,
    /// Runs the `pulldown_cmark` example.
    #[command(name = "pulldown_cmark")]
    PulldownCmark,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Comrak => {
                comrak::run();
            }
            Commands::Markdown => {
                markdown::run();
            }
            Commands::PulldownCmark => {
                pulldown_cmark::run();
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_cli_dispatcher() {
        let _ = Cli::try_parse_from(["example"]);
    }
}
