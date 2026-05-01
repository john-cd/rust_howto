use clap::Parser;
use clap::Subcommand;

mod cow_alternatives;
mod cow_as_function_param;
mod cow_to_borrowed_owned;
mod function_returning_cow;
mod into_cow;
mod modify_cow_in_place;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "cow_alternatives")]
    CowAlternatives,
    #[command(name = "cow_as_function_param")]
    CowAsFunctionParam,
    #[command(name = "cow_to_borrowed_owned")]
    CowToBorrowedOwned,
    #[command(name = "function_returning_cow")]
    FunctionReturningCow,
    #[command(name = "into_cow")]
    IntoCow,
    #[command(name = "modify_cow_in_place")]
    ModifyCowInPlace,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::CowAlternatives => {
                let _ = cow_alternatives::run();
            }
            Commands::CowAsFunctionParam => {
                let _ = cow_as_function_param::run();
            }
            Commands::CowToBorrowedOwned => {
                let _ = cow_to_borrowed_owned::run();
            }
            Commands::FunctionReturningCow => {
                let _ = function_returning_cow::run();
            }
            Commands::IntoCow => {
                let _ = into_cow::run();
            }
            Commands::ModifyCowInPlace => {
                let _ = modify_cow_in_place::run();
            }
        }
    }
}
