use clap::Parser;
use clap::Subcommand;

mod duplicate_name;
mod find_file;
mod globset;
mod ignore;
mod ignore_case;
mod loops;
mod modified;
mod png;
mod sizes;
mod skip_dot;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "duplicate_name")]
    DuplicateName,
    #[command(name = "find_file")]
    FindFile,
    #[command(name = "globset")]
    Globset,
    #[command(name = "ignore")]
    Ignore,
    #[command(name = "ignore_case")]
    IgnoreCase,
    #[command(name = "loops")]
    Loops,
    #[command(name = "modified")]
    Modified,
    #[command(name = "png")]
    Png,
    #[command(name = "sizes")]
    Sizes,
    #[command(name = "skip_dot")]
    SkipDot,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::DuplicateName => {
                let _ = duplicate_name::run();
            }
            Commands::FindFile => {
                let _ = find_file::run();
            }
            Commands::Globset => {
                let _ = globset::run();
            }
            Commands::Ignore => {
                let _ = ignore::run();
            }
            Commands::IgnoreCase => {
                let _ = ignore_case::run();
            }
            Commands::Loops => {
                let _ = loops::run();
            }
            Commands::Modified => {
                let _ = modified::run();
            }
            Commands::Png => {
                let _ = png::run();
            }
            Commands::Sizes => {
                let _ = sizes::run();
            }
            Commands::SkipDot => {
                let _ = skip_dot::run();
            }
        }
    }
}
