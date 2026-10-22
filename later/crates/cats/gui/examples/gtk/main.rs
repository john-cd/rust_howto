//! Runs one of the `gtk` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026

#[cfg(all(feature = "gtk", not(target_os = "windows")))]
use clap::Parser;
#[cfg(all(feature = "gtk", not(target_os = "windows")))]
use clap::Subcommand;

#[cfg(all(feature = "gtk", not(target_os = "windows")))]
mod gtk4;
#[cfg(all(feature = "gtk", not(target_os = "windows")))]
mod relm4;

/// Command-line arguments.
#[cfg(all(feature = "gtk", not(target_os = "windows")))]
#[derive(Parser)]
struct Cli {
    /// The example to run.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// The examples that can be run.
#[cfg(all(feature = "gtk", not(target_os = "windows")))]
#[derive(Subcommand)]
enum Commands {
    /// Runs the `gtk4` example.
    #[command(name = "gtk4")]
    Gtk4,
    /// Runs the `relm4` example.
    #[command(name = "relm4")]
    Relm4,
}

/// Runs the example selected on the command line.
fn main() {
    #[cfg(all(feature = "gtk", not(target_os = "windows")))]
    {
        let cli = Cli::parse();

        if let Some(command) = cli.command {
            match command {
                Commands::Gtk4 => {
                    gtk4::run();
                }
                Commands::Relm4 => {
                    relm4::run();
                }
            }
        }
    }

    #[cfg(all(feature = "gtk", target_os = "windows"))]
    {
        eprintln!("GTK examples are disabled on Windows.");
    }
}
