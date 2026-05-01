use clap::Parser;
use clap::Subcommand;

mod lazy_constant;
mod lazy_static;
mod once_cell;
mod once_cell2;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "lazy_constant")]
    LazyConstant,
    #[command(name = "lazy_static")]
    LazyStatic,
    #[command(name = "once_cell")]
    OnceCell,
    #[command(name = "once_cell2")]
    OnceCell2,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::LazyConstant => {
                lazy_constant::run();
            }
            Commands::LazyStatic => {
                lazy_static::run();
            }
            Commands::OnceCell => {
                once_cell::run();
            }
            Commands::OnceCell2 => {
                once_cell2::run();
            }
        }
    }
}
