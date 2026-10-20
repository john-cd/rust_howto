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
                cow_alternatives::run();
            }
            Commands::CowAsFunctionParam => {
                cow_as_function_param::run();
            }
            Commands::CowToBorrowedOwned => {
                cow_to_borrowed_owned::run();
            }
            Commands::FunctionReturningCow => {
                function_returning_cow::run();
            }
            Commands::IntoCow => {
                into_cow::run();
            }
            Commands::ModifyCowInPlace => {
                modify_cow_in_place::run();
            }
        }
    }
}
