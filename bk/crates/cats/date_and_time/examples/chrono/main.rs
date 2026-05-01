use clap::Parser;
use clap::Subcommand;

mod checked;
mod current;
mod format;
mod parse_string_into_datetime;
mod timestamp;
mod timezone;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "checked")]
    Checked,
    #[command(name = "current")]
    Current,
    #[command(name = "format")]
    Format,
    #[command(name = "parse_string_into_datetime")]
    ParseStringIntoDatetime,
    #[command(name = "timestamp")]
    Timestamp,
    #[command(name = "timezone")]
    Timezone,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Checked => {
                let _ = checked::run();
            }
            Commands::Current => {
                let _ = current::run();
            }
            Commands::Format => {
                let _ = format::run();
            }
            Commands::ParseStringIntoDatetime => {
                let _ = parse_string_into_datetime::run();
            }
            Commands::Timestamp => {
                let _ = timestamp::run();
            }
            Commands::Timezone => {
                let _ = timezone::run();
            }
        }
    }
}
