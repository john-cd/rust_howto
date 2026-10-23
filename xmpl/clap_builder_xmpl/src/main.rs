//! A demo command-line tool built with the `clap` builder API.
//!
//! TODO Reviewed ??-??-2026.

#![warn(clippy::all, rust_2018_idioms)]

mod cli;

/// Parses the command line and prints the configuration and the commands.
fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init(); // Log to stderr.

    let (config, commands) = cli::get_args();

    // Demo use of the configuration and command(s) returned by
    // `get_args`.
    match config.get_verbose() {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    if let Some(c) = config.get_config_file() {
        println!("Value for config file: {}", c.display());
    }

    for cmd in commands {
        match cmd {
            cli::Cmd::None => {
                println!("Command: none");
            }
            cli::Cmd::Open(files) => {
                println!("Command: open {files:?}");
            }
            cli::Cmd::Query(query) => {
                println!("Command: query {query:?}");
            }
            cli::Cmd::Test => {
                println!("Command: test");
            }
        }
    }
}
