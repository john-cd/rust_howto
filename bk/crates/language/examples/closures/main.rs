use clap::Parser;
use clap::Subcommand;

mod closures;
mod closures_as_input_parameters;
mod closures_capture;
mod closures_move;
mod closures_with_type_annotations;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "closures")]
    Closures,
    #[command(name = "closures_as_input_parameters")]
    ClosuresAsInputParameters,
    #[command(name = "closures_capture")]
    ClosuresCapture,
    #[command(name = "closures_move")]
    ClosuresMove,
    #[command(name = "closures_with_type_annotations")]
    ClosuresWithTypeAnnotations,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Closures => {
                let _ = closures::run();
            }
            Commands::ClosuresAsInputParameters => {
                let _ = closures_as_input_parameters::run();
            }
            Commands::ClosuresCapture => {
                let _ = closures_capture::run();
            }
            Commands::ClosuresMove => {
                let _ = closures_move::run();
            }
            Commands::ClosuresWithTypeAnnotations => {
                let _ = closures_with_type_annotations::run();
            }
        }
    }
}
