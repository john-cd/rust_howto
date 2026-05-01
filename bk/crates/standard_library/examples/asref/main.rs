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
                let _ = asref::run();
            }
            Commands::OptionAsref => {
                let _ = option_asref::run();
            }
            Commands::SmartPointerAsref => {
                let _ = smart_pointer_asref::run();
            }
        }
    }
}
