//! Runs one of the `data_parallelism` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod multithreading_rayon;
mod multithreading_rayon_custom;
mod multithreading_rayon_parsort;
mod rayon_any_all;
mod rayon_iter_mut;
mod rayon_map_reduce;
mod rayon_parallel_search;
mod rayon_parallel_sort;
mod rayon_thumbnails;

/// Command-line arguments.
#[derive(Parser)]
struct Cli {
    /// The example to run.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// The examples that can be run.
#[derive(Subcommand)]
enum Commands {
    /// Runs the `multithreading_rayon` example.
    #[command(name = "multithreading_rayon")]
    MultithreadingRayon,
    /// Runs the `multithreading_rayon_custom` example.
    #[command(name = "multithreading_rayon_custom")]
    MultithreadingRayonCustom,
    /// Runs the `multithreading_rayon_parsort` example.
    #[command(name = "multithreading_rayon_parsort")]
    MultithreadingRayonParsort,
    /// Runs the `rayon_any_all` example.
    #[command(name = "rayon_any_all")]
    RayonAnyAll,
    /// Runs the `rayon_iter_mut` example.
    #[command(name = "rayon_iter_mut")]
    RayonIterMut,
    /// Runs the `rayon_map_reduce` example.
    #[command(name = "rayon_map_reduce")]
    RayonMapReduce,
    /// Runs the `rayon_parallel_search` example.
    #[command(name = "rayon_parallel_search")]
    RayonParallelSearch,
    /// Runs the `rayon_parallel_sort` example.
    #[command(name = "rayon_parallel_sort")]
    RayonParallelSort,
    /// Runs the `rayon_thumbnails` example.
    #[command(name = "rayon_thumbnails")]
    RayonThumbnails,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) -> anyhow::Result<()> {
    if let Some(command) = cli.command {
        match command {
            Commands::MultithreadingRayon => {
                multithreading_rayon::run();
            }
            Commands::MultithreadingRayonCustom => {
                multithreading_rayon_custom::run();
            }
            Commands::MultithreadingRayonParsort => {
                multithreading_rayon_parsort::run();
            }
            Commands::RayonAnyAll => {
                rayon_any_all::run();
            }
            Commands::RayonIterMut => {
                rayon_iter_mut::run();
            }
            Commands::RayonMapReduce => {
                rayon_map_reduce::run();
            }
            Commands::RayonParallelSearch => {
                rayon_parallel_search::run();
            }
            Commands::RayonParallelSort => {
                rayon_parallel_sort::run();
            }
            Commands::RayonThumbnails => {
                rayon_thumbnails::run()?;
            }
        }
    }
    Ok(())
}

/// Parses the command line and runs the selected example.
fn main() -> anyhow::Result<()> {
    dispatch(Cli::parse())
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;

    /// Every example is exposed as a subcommand.
    const SUBCOMMANDS: &[&str] = &["multithreading_rayon", "multithreading_rayon_custom", "multithreading_rayon_parsort", "rayon_any_all", "rayon_iter_mut", "rayon_map_reduce", "rayon_parallel_search", "rayon_parallel_sort", "rayon_thumbnails"];

    /// The command-line definition is internally consistent.
    #[test]
    fn test_cli_definition_is_valid() {
        Cli::command().debug_assert();
    }

    /// Each example is reachable through its own subcommand.
    #[test]
    fn test_every_subcommand_parses() {
        for name in SUBCOMMANDS {
            let cli = Cli::try_parse_from(["example", name])
                .unwrap_or_else(|e| panic!("`{name}` should parse: {e}"));
            assert!(cli.command.is_some(), "`{name}` selected no command");
        }
    }

    /// Subcommand names are unique.
    #[test]
    fn test_subcommand_names_are_unique() {
        let mut sorted = SUBCOMMANDS.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), SUBCOMMANDS.len());
    }

    /// Unknown subcommands are rejected.
    #[test]
    fn test_unknown_subcommand_is_rejected() {
        assert!(Cli::try_parse_from(["example", "no-such-example"]).is_err());
    }

    /// Running without a subcommand selects nothing and succeeds.
    #[test]
    fn test_no_subcommand_runs_nothing() {
        let cli = Cli::try_parse_from(["example"]).expect("no subcommand is accepted");
        assert!(cli.command.is_none());
        dispatch(cli).expect("dispatching should succeed");
    }
}
