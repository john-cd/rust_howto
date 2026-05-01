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

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "multithreading_rayon")]
    MultithreadingRayon,
    #[command(name = "multithreading_rayon_custom")]
    MultithreadingRayonCustom,
    #[command(name = "multithreading_rayon_parsort")]
    MultithreadingRayonParsort,
    #[command(name = "rayon_any_all")]
    RayonAnyAll,
    #[command(name = "rayon_iter_mut")]
    RayonIterMut,
    #[command(name = "rayon_map_reduce")]
    RayonMapReduce,
    #[command(name = "rayon_parallel_search")]
    RayonParallelSearch,
    #[command(name = "rayon_parallel_sort")]
    RayonParallelSort,
    #[command(name = "rayon_thumbnails")]
    RayonThumbnails,
}

fn main() {
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
                rayon_thumbnails::run();
            }
        }
    }
}
