#[cfg(any(
    feature = "oracle",
    all(target_os = "linux", feature = "oracle")
))]
use clap::Parser;
#[cfg(any(
    feature = "oracle",
    all(target_os = "linux", feature = "oracle")
))]
use clap::Subcommand;

#[cfg(feature = "oracle")]
mod diesel_oci;
#[cfg(feature = "oracle")]
mod oracle;
#[cfg(all(target_os = "linux", feature = "oracle"))]
mod sibyl;

#[cfg(any(feature = "oracle", all(target_os = "linux", feature = "oracle")))]
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[cfg(any(feature = "oracle", all(target_os = "linux", feature = "oracle")))]
#[derive(Subcommand)]
enum Commands {
    #[cfg(feature = "oracle")]
    #[command(name = "diesel_oci")]
    DieselOci,
    #[cfg(feature = "oracle")]
    #[command(name = "oracle")]
    Oracle,
    #[cfg(all(target_os = "linux", feature = "oracle"))]
    #[command(name = "sibyl")]
    Sibyl,
}

fn main() {
    #[cfg(any(
        feature = "oracle",
        all(target_os = "linux", feature = "oracle")
    ))]
    {
        let cli = Cli::parse();

        if let Some(command) = cli.command {
            match command {
                #[cfg(feature = "oracle")]
                Commands::DieselOci => {
                    let _ = diesel_oci::run();
                }
                #[cfg(feature = "oracle")]
                Commands::Oracle => {
                    let _ = oracle::run();
                }
                #[cfg(all(target_os = "linux", feature = "oracle"))]
                Commands::Sibyl => {
                    let _ = sibyl::run();
                }
            }
        }
    }
}
