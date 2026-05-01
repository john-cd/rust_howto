use clap::Parser;
use clap::Subcommand;

mod queue;
mod stack;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "queue")]
    Queue,
    #[command(name = "stack")]
    Stack,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Queue => {
                queue::run();
            }
            Commands::Stack => {
                stack::run();
            }
        }
    }
}
