#![warn(clippy::all, rust_2018_idioms)]

use std::path::PathBuf;

mod cli;

/// The command passed as argument, if any
#[derive(Default, Clone, Debug)]
pub enum Cmd {
    /// No (implicit or explicit) command was given
    #[default]
    None,
    /// Explicit `open` command or implicit open (given a list of
    /// files)
    Open(Vec<PathBuf>),
    /// `query` command, storing query words
    Query(Vec<String>),
    Test,
}

fn main() {
    let rust_log =
        std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_owned());
    std::env::set_var("RUST_LOG", rust_log);

    env_logger::init(); // Log to stderr

    let (config, cmd) = cli::get_args();

    match config.get_verbose() {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    if let Some(c) = config.get_config_file() {
        println!("Value for config: {}", c.display());
    }

    match cmd {
        Cmd::None => {
            println!("none");
        }
        Cmd::Open(files) => {
            println!("open {:?}", files);
        }
        Cmd::Query(query) => {
            println!("query {:?}", query);
        }
        Cmd::Test => {
            println!("test");
        }
    }
}
