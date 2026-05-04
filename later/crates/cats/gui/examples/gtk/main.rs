#[cfg(all(feature = "gtk", not(target_os = "windows")))]
use clap::Parser;
#[cfg(all(feature = "gtk", not(target_os = "windows")))]
use clap::Subcommand;

#[cfg(all(feature = "gtk", not(target_os = "windows")))]
mod gtk4;
#[cfg(all(feature = "gtk", not(target_os = "windows")))]
mod relm4;

#[cfg(all(feature = "gtk", not(target_os = "windows")))]
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[cfg(all(feature = "gtk", not(target_os = "windows")))]
#[derive(Subcommand)]
enum Commands {
    #[command(name = "gtk4")]
    Gtk4,
    #[command(name = "relm4")]
    Relm4,
}

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
