use clap::Parser;
use clap::Subcommand;

mod duplicate_name;
mod find_file;
mod globset;
mod ignore;
mod ignore_case;
#[cfg(target_os = "linux")]
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
    #[cfg(target_os = "linux")]
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
                duplicate_name::run();
            }
            Commands::FindFile => {
                find_file::run();
            }
            Commands::Globset => {
                globset::run();
            }
            Commands::Ignore => {
                ignore::run();
            }
            Commands::IgnoreCase => {
                ignore_case::run();
            }
            #[cfg(target_os = "linux")]
            Commands::Loops => {
                loops::run();
            }
            Commands::Modified => {
                modified::run();
            }
            Commands::Png => {
                png::run();
            }
            Commands::Sizes => {
                sizes::run();
            }
            Commands::SkipDot => {
                skip_dot::run();
            }
        }
    }
}
