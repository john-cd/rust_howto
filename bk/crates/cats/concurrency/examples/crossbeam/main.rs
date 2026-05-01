use clap::Parser;
use clap::Subcommand;

mod crossbeam_complex;
mod crossbeam_spawn;
mod crossbeam_spsc;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "crossbeam_complex")]
    CrossbeamComplex,
    #[command(name = "crossbeam_spawn")]
    CrossbeamSpawn,
    #[command(name = "crossbeam_spsc")]
    CrossbeamSpsc,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::CrossbeamComplex => {
                let _ = crossbeam_complex::run();
            }
            Commands::CrossbeamSpawn => {
                let _ = crossbeam_spawn::run();
            }
            Commands::CrossbeamSpsc => {
                let _ = crossbeam_spsc::run();
            }
        }
    }
}
