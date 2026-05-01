use clap::Parser;
use clap::Subcommand;

mod anyhow;
mod backtrace;
mod color_eyre;
mod error_handling;
mod miette;
mod panic;
mod question_mark;
mod question_mark2;
mod retain;
mod thiserror;
mod thiserror2;
mod unwrap;
mod unwrap_or_else;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "anyhow")]
    Anyhow,
    #[command(name = "backtrace")]
    Backtrace,
    #[command(name = "color_eyre")]
    ColorEyre,
    #[command(name = "error_handling")]
    ErrorHandling,
    #[command(name = "miette")]
    Miette,
    #[command(name = "panic")]
    Panic,
    #[command(name = "question_mark")]
    QuestionMark,
    #[command(name = "question_mark2")]
    QuestionMark2,
    #[command(name = "retain")]
    Retain,
    #[command(name = "thiserror")]
    Thiserror,
    #[command(name = "thiserror2")]
    Thiserror2,
    #[command(name = "unwrap")]
    Unwrap,
    #[command(name = "unwrap_or_else")]
    UnwrapOrElse,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Anyhow => {
                let _ = anyhow::run();
            }
            Commands::Backtrace => {
                let _ = backtrace::run();
            }
            Commands::ColorEyre => {
                let _ = color_eyre::run();
            }
            Commands::ErrorHandling => {
                let _ = error_handling::run();
            }
            Commands::Miette => {
                let _ = miette::run();
            }
            Commands::Panic => {
                let _ = panic::run();
            }
            Commands::QuestionMark => {
                let _ = question_mark::run();
            }
            Commands::QuestionMark2 => {
                let _ = question_mark2::run();
            }
            Commands::Retain => {
                let _ = retain::run();
            }
            Commands::Thiserror => {
                let _ = thiserror::run();
            }
            Commands::Thiserror2 => {
                let _ = thiserror2::run();
            }
            Commands::Unwrap => {
                let _ = unwrap::run();
            }
            Commands::UnwrapOrElse => {
                let _ = unwrap_or_else::run();
            }
        }
    }
}
