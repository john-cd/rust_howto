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
                let _ = const_generics::run();
            }
            Commands::ConstGenerics2 => {
                let _ = const_generics2::run();
            }
            Commands::GenericLifetime => {
                let _ = generic_lifetime::run();
            }
            Commands::GenericTypeParameter => {
                let _ = generic_type_parameter::run();
            }
        }
    }
}
