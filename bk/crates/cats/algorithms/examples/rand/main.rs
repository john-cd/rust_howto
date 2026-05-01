use clap::Parser;
use clap::Subcommand;

mod fastrand;
mod rand1;
mod rand_alphanumeric;
mod rand_custom;
mod rand_distr;
mod rand_password;
mod rand_range;
mod rand_range1;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "fastrand")]
    Fastrand,
    #[command(name = "rand1")]
    Rand1,
    #[command(name = "rand_alphanumeric")]
    RandAlphanumeric,
    #[command(name = "rand_custom")]
    RandCustom,
    #[command(name = "rand_distr")]
    RandDistr,
    #[command(name = "rand_password")]
    RandPassword,
    #[command(name = "rand_range")]
    RandRange,
    #[command(name = "rand_range1")]
    RandRange1,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Fastrand => {
                let _ = fastrand::run();
            }
            Commands::Rand1 => {
                let _ = rand1::run();
            }
            Commands::RandAlphanumeric => {
                let _ = rand_alphanumeric::run();
            }
            Commands::RandCustom => {
                let _ = rand_custom::run();
            }
            Commands::RandDistr => {
                let _ = rand_distr::run();
            }
            Commands::RandPassword => {
                let _ = rand_password::run();
            }
            Commands::RandRange => {
                let _ = rand_range::run();
            }
            Commands::RandRange1 => {
                let _ = rand_range1::run();
            }
        }
    }
}
