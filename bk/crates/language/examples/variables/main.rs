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
                let _ = destructuring::run();
            }
            Commands::Shadowing => {
                let _ = shadowing::run();
            }
            Commands::Vars => {
                let _ = vars::run();
            }
            Commands::Vars2 => {
                let _ = vars2::run();
            }
        }
    }
}
