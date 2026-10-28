//! Runs one of the `visibility` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod private_access;
mod pub_keyword;
mod public_access;
mod public_by_default;
mod visibility_external_code;
mod visibility_pub_crate;
mod visibility_scope;

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
    /// Runs the `private_access` example.
    #[command(name = "private_access")]
    PrivateAccess,
    /// Runs the `pub_keyword` example.
    #[command(name = "pub_keyword")]
    PubKeyword,
    /// Runs the `public_access` example.
    #[command(name = "public_access")]
    PublicAccess,
    /// Runs the `public_by_default` example.
    #[command(name = "public_by_default")]
    PublicByDefault,
    /// Runs the `visibility_external_code` example.
    #[command(name = "visibility_external_code")]
    VisibilityExternalCode,
    /// Runs the `visibility_pub_crate` example.
    #[command(name = "visibility_pub_crate")]
    VisibilityPubCrate,
    /// Runs the `visibility_scope` example.
    #[command(name = "visibility_scope")]
    VisibilityScope,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::PrivateAccess => {
                private_access::run();
            }
            Commands::PubKeyword => {
                pub_keyword::run();
            }
            Commands::PublicAccess => {
                public_access::run();
            }
            Commands::PublicByDefault => {
                public_by_default::run();
            }
            Commands::VisibilityExternalCode => {
                visibility_external_code::run();
            }
            Commands::VisibilityPubCrate => {
                visibility_pub_crate::run();
            }
            Commands::VisibilityScope => {
                visibility_scope::run();
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
