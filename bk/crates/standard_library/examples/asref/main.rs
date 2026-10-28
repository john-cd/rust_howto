//! Runs one of the `asref` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod asref;
mod option_asref;
mod smart_pointer_asref;

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
    /// Runs the `asref` example.
    #[command(name = "asref")]
    Asref,
    /// Runs the `option_asref` example.
    #[command(name = "option_asref")]
    OptionAsref,
    /// Runs the `smart_pointer_asref` example.
    #[command(name = "smart_pointer_asref")]
    SmartPointerAsref,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Asref => {
                asref::run();
            }
            Commands::OptionAsref => {
                option_asref::run();
            }
            Commands::SmartPointerAsref => {
                smart_pointer_asref::run();
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
