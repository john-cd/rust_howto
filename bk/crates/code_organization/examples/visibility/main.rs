use clap::Parser;
use clap::Subcommand;

mod private_access;
mod pub_keyword;
mod public_access;
mod public_by_default;
mod visibility_external_code;
mod visibility_pub_crate;
mod visibility_scope;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "private_access")]
    PrivateAccess,
    #[command(name = "pub_keyword")]
    PubKeyword,
    #[command(name = "public_access")]
    PublicAccess,
    #[command(name = "public_by_default")]
    PublicByDefault,
    #[command(name = "visibility_external_code")]
    VisibilityExternalCode,
    #[command(name = "visibility_pub_crate")]
    VisibilityPubCrate,
    #[command(name = "visibility_scope")]
    VisibilityScope,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::PrivateAccess => {
                let _ = private_access::run();
            }
            Commands::PubKeyword => {
                let _ = pub_keyword::run();
            }
            Commands::PublicAccess => {
                let _ = public_access::run();
            }
            Commands::PublicByDefault => {
                let _ = public_by_default::run();
            }
            Commands::VisibilityExternalCode => {
                let _ = visibility_external_code::run();
            }
            Commands::VisibilityPubCrate => {
                let _ = visibility_pub_crate::run();
            }
            Commands::VisibilityScope => {
                let _ = visibility_scope::run();
            }
        }
    }
}
