use clap::Parser;
use clap::Subcommand;

mod diverging_functions;
mod function_pointers;
mod functions;
mod generic_functions;
mod generic_functions2;
mod generic_functions3;
mod return_reference;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "diverging_functions")]
    DivergingFunctions,
    #[command(name = "function_pointers")]
    FunctionPointers,
    #[command(name = "functions")]
    Functions,
    #[command(name = "generic_functions")]
    GenericFunctions,
    #[command(name = "generic_functions2")]
    GenericFunctions2,
    #[command(name = "generic_functions3")]
    GenericFunctions3,
    #[command(name = "return_reference")]
    ReturnReference,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::DivergingFunctions => {
                let _ = diverging_functions::run();
            }
            Commands::FunctionPointers => {
                let _ = function_pointers::run();
            }
            Commands::Functions => {
                let _ = functions::run();
            }
            Commands::GenericFunctions => {
                let _ = generic_functions::run();
            }
            Commands::GenericFunctions2 => {
                let _ = generic_functions2::run();
            }
            Commands::GenericFunctions3 => {
                let _ = generic_functions3::run();
            }
            Commands::ReturnReference => {
                let _ = return_reference::run();
            }
        }
    }
}
