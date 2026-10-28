//! Runs one of the `other_strings` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod bstr;
mod cstring;
mod osstring;
mod ustr;

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
    /// Runs the `bstr` example.
    #[command(name = "bstr")]
    Bstr,
    /// Runs the `cstring` example.
    #[command(name = "cstring")]
    Cstring,
    /// Runs the `osstring` example.
    #[command(name = "osstring")]
    Osstring,
    /// Runs the `ustr` example.
    #[command(name = "ustr")]
    Ustr,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Bstr => {
                bstr::run();
            }
            Commands::Cstring => {
                cstring::run();
            }
            Commands::Osstring => {
                osstring::run();
            }
            Commands::Ustr => {
                ustr::run();
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
