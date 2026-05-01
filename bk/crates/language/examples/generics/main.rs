use clap::Parser;
use clap::Subcommand;

mod const_generics;
mod const_generics2;
mod generic_lifetime;
mod generic_type_parameter;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "const_generics")]
    ConstGenerics,
    #[command(name = "const_generics2")]
    ConstGenerics2,
    #[command(name = "generic_lifetime")]
    GenericLifetime,
    #[command(name = "generic_type_parameter")]
    GenericTypeParameter,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::ConstGenerics => {
                const_generics::run();
            }
            Commands::ConstGenerics2 => {
                const_generics2::run();
            }
            Commands::GenericLifetime => {
                generic_lifetime::run();
            }
            Commands::GenericTypeParameter => {
                generic_type_parameter::run();
            }
        }
    }
}
