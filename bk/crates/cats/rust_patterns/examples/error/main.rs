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

fn main() -> ::anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Anyhow => {
                anyhow::run()?;
            }
            Commands::Backtrace => {
                backtrace::run()
                    .map_err(|e| ::anyhow::anyhow!(e.to_string()))?;
            }
            Commands::ColorEyre => {
                color_eyre::run()
                    .map_err(|e| ::anyhow::anyhow!(e.to_string()))?;
            }
            Commands::ErrorHandling => {
                error_handling::run();
            }
            Commands::Miette => {
                miette::run().map_err(|e| ::anyhow::anyhow!(e.to_string()))?;
            }
            Commands::Panic => {
                panic::run();
            }
            Commands::QuestionMark => {
                question_mark::run();
            }
            Commands::QuestionMark2 => {
                question_mark2::run();
            }
            Commands::Retain => {
                retain::run()?;
            }
            Commands::Thiserror => {
                thiserror::run()
                    .map_err(|e| ::anyhow::anyhow!(e.to_string()))?;
            }
            Commands::Thiserror2 => {
                thiserror2::run();
            }
            Commands::Unwrap => {
                unwrap::run();
            }
            Commands::UnwrapOrElse => {
                unwrap_or_else::run();
            }
        }
    }

    Ok(())
}
