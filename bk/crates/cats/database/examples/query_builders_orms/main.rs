#[cfg(any(feature = "diesel", feature = "sea_orm", feature = "sqlx"))]
use clap::Parser;
#[cfg(any(feature = "diesel", feature = "sea_orm", feature = "sqlx"))]
use clap::Subcommand;

#[cfg(feature = "diesel")]
mod diesel1;
#[cfg(feature = "sea_orm")]
mod sea_orm;
#[cfg(feature = "sea_orm")]
mod seaography;
#[cfg(feature = "sqlx")]
mod sqlx;

#[cfg(any(feature = "diesel", feature = "sea_orm", feature = "sqlx"))]
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[cfg(any(feature = "diesel", feature = "sea_orm", feature = "sqlx"))]
#[derive(Subcommand)]
enum Commands {
    #[cfg(feature = "diesel")]
    #[command(name = "diesel1")]
    Diesel1,
    #[cfg(feature = "sea_orm")]
    #[command(name = "sea_orm")]
    SeaOrm,
    #[cfg(feature = "sea_orm")]
    #[command(name = "seaography")]
    Seaography,
    #[cfg(feature = "sqlx")]
    #[command(name = "sqlx")]
    Sqlx,
}

fn main() {
    #[cfg(any(feature = "diesel", feature = "sea_orm", feature = "sqlx"))]
    {
        let cli = Cli::parse();

        if let Some(command) = cli.command {
            match command {
                #[cfg(feature = "diesel")]
                Commands::Diesel1 => {
                    let _ = diesel1::run();
                }
                #[cfg(feature = "sea_orm")]
                Commands::SeaOrm => {
                    let _ = sea_orm::run();
                }
                #[cfg(feature = "sea_orm")]
                Commands::Seaography => {
                    let _ = seaography::run();
                }
                #[cfg(feature = "sqlx")]
                Commands::Sqlx => {
                    let _ = sqlx::run();
                }
            }
        }
    }
}
