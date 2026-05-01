use clap::Parser;
use clap::Subcommand;

mod bytemuck;
mod zerocopy;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "bytemuck")]
    Bytemuck,
    #[command(name = "zerocopy")]
    Zerocopy,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Bytemuck => {
                bytemuck::run();
            }
            Commands::Zerocopy => {
                zerocopy::run();
            }
        }
    }
}
