use clap::Parser;
use clap::Subcommand;

mod pub_use;
mod use1;
mod use2;
mod use3;
mod use_external_crate;
mod use_shortcuts;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "pub_use")]
    PubUse,
    #[command(name = "use1")]
    Use1,
    #[command(name = "use2")]
    Use2,
    #[command(name = "use3")]
    Use3,
    #[command(name = "use_external_crate")]
    UseExternalCrate,
    #[command(name = "use_shortcuts")]
    UseShortcuts,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::PubUse => {
                pub_use::run();
            }
            Commands::Use1 => {
                use1::run();
            }
            Commands::Use2 => {
                use2::run();
            }
            Commands::Use3 => {
                use3::run();
            }
            Commands::UseExternalCrate => {
                use_external_crate::run();
            }
            Commands::UseShortcuts => {
                use_shortcuts::run();
            }
        }
    }
}
