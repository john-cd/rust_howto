use clap::Parser;
use clap::Subcommand;

mod approx;
mod tests_custom_message;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "approx")]
    Approx,
    #[command(name = "tests_custom_message")]
    TestsCustomMessage,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Approx => {
                approx::run();
            }
            Commands::TestsCustomMessage => {
                tests_custom_message::run();
            }
        }
    }
}
