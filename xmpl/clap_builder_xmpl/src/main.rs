#![warn(clippy::all, rust_2018_idioms)]

mod cli;

fn main() {
    let rust_log =
        std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_owned());
    std::env::set_var("RUST_LOG", rust_log);

    env_logger::init(); // Log to stderr

    let (config, cmds) = cli::get_args();

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

    for cmd in cmds {
        match cmd {
            cli::Cmd::None => {
                println!("Command: none");
            }
            cli::Cmd::Open(files) => {
                println!("Command: open {:?}", files);
            }
            cli::Cmd::Query(query) => {
                println!("Command: query {:?}", query);
            }
            cli::Cmd::Test => {
                println!("Command: test");
            }
        }
    }
}
