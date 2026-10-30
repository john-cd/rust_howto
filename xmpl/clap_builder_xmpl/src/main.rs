//! A demo command-line tool built with the `clap` builder API.
//!
//! TODO Reviewed ??-??-2026.

#![warn(clippy::all, rust_2018_idioms)]

mod cli;

/// Parses the command line and prints the configuration and the commands.
fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init(); // Log to stderr.

    for line in describe(cli::get_args()) {
        println!("{line}");
    }
}

/// Describes the configuration and the commands returned by `get_args`.
fn describe((config, commands): (cli::Config, Vec<cli::Cmd>)) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(
        match config.get_verbose() {
            0 => "Debug mode is off",
            1 => "Debug mode is kind of on",
            2 => "Debug mode is on",
            _ => "Don't be crazy",
        }
        .to_owned(),
    );

    if let Some(c) = config.get_config_file() {
        lines.push(format!("Value for config file: {}", c.display()));
    }

    for cmd in commands {
        lines.push(match cmd {
            cli::Cmd::None => "Command: none".to_owned(),
            cli::Cmd::Open(files) => format!("Command: open {files:?}"),
            cli::Cmd::Query(query) => format!("Command: query {query:?}"),
            cli::Cmd::Test => "Command: test".to_owned(),
        });
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the demo on the given arguments.
    fn describe_args(args: &[&str]) -> Vec<String> {
        describe(cli::get_args_from(args))
    }

    /// Checks that `main` runs: the test harness's own arguments are not
    /// valid for the demo CLI, so it only works when they are ignored.
    #[test]
    #[ignore = "main() parses the real process arguments, which belong to the test harness"]
    fn main_runs() {
        main();
    }

    /// Checks the output without arguments.
    #[test]
    fn no_arguments() {
        assert_eq!(
            describe_args(&["demo"]),
            ["Debug mode is off", "Command: none"]
        );
    }

    /// Checks the verbosity levels.
    #[test]
    fn verbosity_levels() {
        assert_eq!(
            describe_args(&["demo", "-v"])[0],
            "Debug mode is kind of on"
        );
        assert_eq!(describe_args(&["demo", "-vv"])[0], "Debug mode is on");
        assert_eq!(describe_args(&["demo", "-vvv"])[0], "Don't be crazy");
    }

    /// Checks the config file and the `open` command.
    #[test]
    fn config_and_open() {
        let lines = describe_args(&["demo", "-c", "a.toml", "open", "x.csv"]);
        assert_eq!(lines[1], "Value for config file: a.toml");
        assert_eq!(lines[2], r#"Command: open ["x.csv"]"#);
    }

    /// Checks the `query` and `test` commands.
    #[test]
    fn query_and_test() {
        assert_eq!(
            describe_args(&["demo", "query", "a", "b"]).last(),
            Some(&r#"Command: query ["a", "b"]"#.to_owned())
        );
        assert_eq!(
            describe_args(&["demo", "test"]).last(),
            Some(&"Command: test".to_owned())
        );
    }
}
