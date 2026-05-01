#[cfg(feature = "cassandra")]
use clap::Parser;
#[cfg(feature = "cassandra")]
use clap::Subcommand;

#[cfg(feature = "cassandra")]
mod cassandra_protocol;
#[cfg(feature = "cassandra")]
mod cdrs_tokio;

#[cfg(feature = "cassandra")]
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[cfg(feature = "cassandra")]
#[derive(Subcommand)]
enum Commands {
    #[cfg(feature = "cassandra")]
    #[command(name = "cassandra_protocol")]
    CassandraProtocol,
    #[cfg(feature = "cassandra")]
    #[command(name = "cdrs_tokio")]
    CdrsTokio,
}

fn main() {
    #[cfg(feature = "cassandra")]
    {
        let cli = Cli::parse();

        if let Some(command) = cli.command {
            match command {
                #[cfg(feature = "cassandra")]
                Commands::CassandraProtocol => {
                    cassandra_protocol::run();
                }
                #[cfg(feature = "cassandra")]
                Commands::CdrsTokio => {
                    cdrs_tokio::run();
                }
            }
        }
    }
}
