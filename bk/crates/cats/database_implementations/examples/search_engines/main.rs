#[cfg(any(feature = "meilisearch", feature = "tantivy"))]
use clap::Parser;
#[cfg(any(feature = "meilisearch", feature = "tantivy"))]
use clap::Subcommand;

#[cfg(feature = "meilisearch")]
mod meilisearch;
#[cfg(feature = "tantivy")]
mod tantivy;

#[cfg(any(feature = "meilisearch", feature = "tantivy"))]
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[cfg(any(feature = "meilisearch", feature = "tantivy"))]
#[derive(Subcommand)]
enum Commands {
    #[cfg(feature = "meilisearch")]
    #[command(name = "meilisearch")]
    Meilisearch,
    #[cfg(feature = "tantivy")]
    #[command(name = "tantivy")]
    Tantivy,
}

fn main() -> anyhow::Result<()> {
    #[cfg(any(feature = "meilisearch", feature = "tantivy"))]
    {
        let cli = Cli::parse();

        if let Some(command) = cli.command {
            match command {
                #[cfg(feature = "meilisearch")]
                Commands::Meilisearch => {
                    meilisearch::run()?;
                }
                #[cfg(feature = "tantivy")]
                Commands::Tantivy => {
                    tantivy::run().map_err(|e| anyhow::anyhow!(e.to_string()))?;
                }
            }
        }
    }

    Ok(())
}
