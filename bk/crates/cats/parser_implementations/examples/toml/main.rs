use clap::Parser;
use clap::Subcommand;

mod basic_toml;
mod toml;
mod toml_edit;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "basic_toml")]
    BasicToml,
    #[command(name = "toml")]
    Toml,
    #[command(name = "toml_edit")]
    TomlEdit,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::BasicToml => {
                basic_toml::run();
            }
            Commands::Toml => {
                toml::run();
            }
            Commands::TomlEdit => {
                toml_edit::run();
            }
        }
    }
}
