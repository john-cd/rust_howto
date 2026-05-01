use clap::Parser;
use clap::Subcommand;

mod box_basics;
mod box_deref;
mod box_dst;
mod box_recursive;
mod box_trait_objects;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "box_basics")]
    BoxBasics,
    #[command(name = "box_deref")]
    BoxDeref,
    #[command(name = "box_dst")]
    BoxDst,
    #[command(name = "box_recursive")]
    BoxRecursive,
    #[command(name = "box_trait_objects")]
    BoxTraitObjects,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::BoxBasics => {
                let _ = box_basics::run();
            }
            Commands::BoxDeref => {
                let _ = box_deref::run();
            }
            Commands::BoxDst => {
                let _ = box_dst::run();
            }
            Commands::BoxRecursive => {
                let _ = box_recursive::run();
            }
            Commands::BoxTraitObjects => {
                let _ = box_trait_objects::run();
            }
        }
    }
}
