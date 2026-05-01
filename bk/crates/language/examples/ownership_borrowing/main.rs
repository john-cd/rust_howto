use clap::Parser;
use clap::Subcommand;

mod borrowing_function;
mod borrowing_function_mutable;
mod borrowing_immutable;
mod clone;
mod copy;
mod move1;
mod move_function;
mod read_write_lock;
mod scope;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "borrowing_function")]
    BorrowingFunction,
    #[command(name = "borrowing_function_mutable")]
    BorrowingFunctionMutable,
    #[command(name = "borrowing_immutable")]
    BorrowingImmutable,
    #[command(name = "clone")]
    Clone,
    #[command(name = "copy")]
    Copy,
    #[command(name = "move1")]
    Move1,
    #[command(name = "move_function")]
    MoveFunction,
    #[command(name = "read_write_lock")]
    ReadWriteLock,
    #[command(name = "scope")]
    Scope,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::BorrowingFunction => {
                let _ = borrowing_function::run();
            }
            Commands::BorrowingFunctionMutable => {
                let _ = borrowing_function_mutable::run();
            }
            Commands::BorrowingImmutable => {
                let _ = borrowing_immutable::run();
            }
            Commands::Clone => {
                let _ = clone::run();
            }
            Commands::Copy => {
                let _ = copy::run();
            }
            Commands::Move1 => {
                let _ = move1::run();
            }
            Commands::MoveFunction => {
                let _ = move_function::run();
            }
            Commands::ReadWriteLock => {
                let _ = read_write_lock::run();
            }
            Commands::Scope => {
                let _ = scope::run();
            }
        }
    }
}
