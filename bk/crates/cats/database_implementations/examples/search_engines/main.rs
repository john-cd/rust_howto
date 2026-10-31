//! Runs one of the `search_engines` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

#[cfg(any(feature = "meilisearch", feature = "tantivy"))]
use clap::Parser;
#[cfg(any(feature = "meilisearch", feature = "tantivy"))]
use clap::Subcommand;

#[cfg(feature = "meilisearch")]
mod meilisearch;
#[cfg(feature = "tantivy")]
mod tantivy;

/// Command-line arguments.
#[cfg(any(feature = "meilisearch", feature = "tantivy"))]
#[derive(Parser)]
struct Cli {
    /// The example to run.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// The examples that can be run.
#[cfg(any(feature = "meilisearch", feature = "tantivy"))]
#[derive(Subcommand)]
enum Commands {
    /// Runs the `meilisearch` example.
    #[cfg(feature = "meilisearch")]
    #[command(name = "meilisearch")]
    Meilisearch,
    /// Runs the `tantivy` example.
    #[cfg(feature = "tantivy")]
    #[command(name = "tantivy")]
    Tantivy,
}

/// Runs the example selected on the command line.
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
                    tantivy::run()
                        .map_err(|e| anyhow::anyhow!(e.to_string()))?;
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Without any example feature, `main` has nothing to run and succeeds.
    #[cfg(not(any(feature = "meilisearch", feature = "tantivy")))]
    #[test]
    fn test_main_is_a_no_op_without_features() {
        assert!(main().is_ok());
    }

    #[cfg(feature = "meilisearch")]
    #[test]
    fn test_meilisearch_subcommand_parses() {
        assert!(Cli::try_parse_from(["example", "meilisearch"]).is_ok());
    }

    #[cfg(feature = "tantivy")]
    #[test]
    fn test_tantivy_subcommand_parses() {
        assert!(Cli::try_parse_from(["example", "tantivy"]).is_ok());
    }

    /// Unknown subcommands are rejected.
    #[cfg(any(feature = "meilisearch", feature = "tantivy"))]
    #[test]
    fn test_unknown_subcommand_is_rejected() {
        assert!(Cli::try_parse_from(["example", "no-such-example"]).is_err());
    }
}
