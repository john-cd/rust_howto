// TODO Reviewed ??-??-2026
use clap::Parser;
use clap::Subcommand;

mod checked;
mod current;
mod format;
mod parse_string_into_datetime;
mod timestamp;
mod timezone;

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
    /// Runs the `checked` example.
    #[command(name = "checked")]
    Checked,
    /// Runs the `current` example.
    #[command(name = "current")]
    Current,
    /// Runs the `format` example.
    #[command(name = "format")]
    Format,
    /// Runs the `parse_string_into_datetime` example.
    #[command(name = "parse_string_into_datetime")]
    ParseStringIntoDatetime,
    /// Runs the `timestamp` example.
    #[command(name = "timestamp")]
    Timestamp,
    /// Runs the `timezone` example.
    #[command(name = "timezone")]
    Timezone,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Checked => {
                checked::run();
            }
            Commands::Current => {
                current::run();
            }
            Commands::Format => {
                format::run();
            }
            Commands::ParseStringIntoDatetime => {
                parse_string_into_datetime::run()
                    .map_err(anyhow::Error::msg)?;
            }
            Commands::Timestamp => {
                timestamp::run();
            }
            Commands::Timezone => {
                timezone::run();
            }
        }
    }
    Ok(())
}
