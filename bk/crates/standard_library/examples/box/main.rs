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
                box_basics::run();
            }
            Commands::BoxDeref => {
                box_deref::run();
            }
            Commands::BoxDst => {
                box_dst::run();
            }
            Commands::BoxRecursive => {
                box_recursive::run();
            }
            Commands::BoxTraitObjects => {
                box_trait_objects::run();
            }
        }
    }
}
