use clap::Parser;
use clap::Subcommand;

mod async_main;
mod main_fn;
mod main_fn_with_result;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "async_main")]
    AsyncMain,
    #[command(name = "main_fn")]
    MainFn,
    #[command(name = "main_fn_with_result")]
    MainFnWithResult,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::AsyncMain => {
                async_main::run();
            }
            Commands::MainFn => {
                main_fn::run();
            }
            Commands::MainFnWithResult => {
                main_fn_with_result::run();
            }
        }
    }
}
