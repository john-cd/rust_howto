use clap::Parser;
use clap::Subcommand;

mod add_complex;
mod create_complex;
mod mathematical_functions;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "add_complex")]
    AddComplex,
    #[command(name = "create_complex")]
    CreateComplex,
    #[command(name = "mathematical_functions")]
    MathematicalFunctions,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::AddComplex => {
                add_complex::run();
            }
            Commands::CreateComplex => {
                create_complex::run();
            }
            Commands::MathematicalFunctions => {
                mathematical_functions::run();
            }
        }
    }
}
