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
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

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
