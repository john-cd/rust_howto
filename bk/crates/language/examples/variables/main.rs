use clap::Parser;
use clap::Subcommand;

mod destructuring;
mod shadowing;
mod vars;
mod vars2;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "destructuring")]
    Destructuring,
    #[command(name = "shadowing")]
    Shadowing,
    #[command(name = "vars")]
    Vars,
    #[command(name = "vars2")]
    Vars2,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Destructuring => {
                destructuring::run();
            }
            Commands::Shadowing => {
                shadowing::run();
            }
            Commands::Vars => {
                vars::run();
            }
            Commands::Vars2 => {
                vars2::run();
            }
        }
    }
}
