use clap::Parser;
use clap::Subcommand;

mod asref;
mod option_asref;
mod smart_pointer_asref;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "asref")]
    Asref,
    #[command(name = "option_asref")]
    OptionAsref,
    #[command(name = "smart_pointer_asref")]
    SmartPointerAsref,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Asref => {
                asref::run();
            }
            Commands::OptionAsref => {
                option_asref::run();
            }
            Commands::SmartPointerAsref => {
                smart_pointer_asref::run();
            }
        }
    }
}
