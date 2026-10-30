//! The arguments of the command line that apply to the whole application.
//!
//! TODO Reviewed ??-??-2026.

use std::path::PathBuf;

use clap::Arg;
use clap::ArgAction;
use clap::ArgMatches;
use clap::ValueHint;
use clap::value_parser;

// (APP-LEVEL) ARGUMENTS -------------------------------

/// Global `--config FILE` flag.
///
/// Example: `cargo r -- --conf config.toml`
pub(super) fn arg_config() -> Arg {
    Arg::new("config")
        .short('c') // -c
        .long("config") // --config
        .alias("configuration") // --configuration (hidden alias - use `visible_alias` to show in the help message)
        // default .num_args(1)
        .value_name("FILE") // FILE, not <config>, is the placeholder for the argument's value in the help message / usage.
        .value_parser(value_parser!(PathBuf)) // Parse and validate the FILE value before storing it into ArgMatches as the given type.
        .value_hint(ValueHint::FilePath) // Provide the shell a hint about how to complete this argument.
        .help("Provides a custom config file")
        // default .required(false) // The argument does not need to be present.
        // default .action(ArgAction::Set)
        .env("TOOL_CONFIG_FILE") // Read from the environment variable when the argument is not present.
        .global(true) // Make the --config flag available to all subcommands.
}

/// Returns the path given with `--config`, if any.
pub(super) fn get_arg_config(matches: &ArgMatches) -> Option<PathBuf> {
    matches.get_one::<PathBuf>("config").cloned()
}

/// Positional argument for one or more file paths.
///
/// Example: `cargo r -- file1.csv file2.csv`
pub(super) fn arg_filepaths() -> Arg {
    Arg::new("filepaths")
        .value_parser(value_parser!(PathBuf))
        .help("File names or paths")
        .action(ArgAction::Append) // Allow passing multiple filepaths.
}

/// Returns the paths of the files to open.
///
/// Returns `None` if the argument(s) are not present.
pub(super) fn get_arg_filepaths(matches: &ArgMatches) -> Option<Vec<PathBuf>> {
    let m = matches
        .get_many::<PathBuf>("filepaths")?
        .cloned()
        .collect::<Vec<PathBuf>>();
    Some(m)
}

/// Global `--verbose / -v` flag that can be added more than once.
///
/// Example: `cargo r -- -vv`
pub(super) fn arg_verbose() -> Arg {
    Arg::new("verbose")
        .short('v') // -v
        .long("verbose") // --verbose
        .help("Verbose output")
        .action(ArgAction::Count)
        // .default_value("0")
        .env("TOOL_VERBOSE") // example: TOOL_VERBOSE=2 cargo run
        .global(true)
}

/// Returns the count of verbose flags i.e. the verbosity level.
///
/// Returns zero if the flag is not present.
pub(super) fn get_arg_verbose(matches: &ArgMatches) -> u8 {
    matches.get_count("verbose")
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    use super::*;

    use crate::cli::cli;

    /// Checks Clap's environment-backed arguments in a child with explicit values.
    fn check_environment(config: Option<&str>, verbose: Option<&str>) -> std::io::Result<()> {
        let mut command = Command::new(std::env::current_exe()?);
        command
            .args(["child_checks_controlled_environment", "--ignored"])
            .env_remove("TOOL_CONFIG_FILE")
            .env_remove("TOOL_VERBOSE")
            .env("EXPECTED_CONFIG", config.unwrap_or("<unset>"))
            .env("EXPECTED_VERBOSE", verbose.unwrap_or("0"));
        if let Some(config) = config {
            command.env("TOOL_CONFIG_FILE", config);
        }
        if let Some(verbose) = verbose {
            command.env("TOOL_VERBOSE", verbose);
        }

        let output = command.output()?;
        assert!(
            output.status.success(),
            "child environment check failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        Ok(())
    }

    /// Verify that the global argument "config" requires a value.
    #[test]
    fn test_arg_config_malformed() {
        let m = cli().try_get_matches_from(vec!["prog", "--config"]); // no value provided.
        assert!(m.is_err());
    }

    /// Test the global argument "config".
    #[test]
    fn test_arg_config_short() {
        let m = cli().get_matches_from(vec!["prog", "-c", "config.toml"]);
        assert_eq!(
            get_arg_config(&m).map(|pb| format!("{}", pb.display())),
            Some("config.toml".into())
        );
    }

    /// Test the global argument "config", long form.
    #[test]
    fn test_arg_config_long() {
        let m = cli().get_matches_from(vec!["prog", "--config", "config.toml"]);
        assert_eq!(
            get_arg_config(&m).map(|pb| format!("{}", pb.display())),
            Some("config.toml".into())
        );
    }

    /// Test the global argument "config", alias form.
    #[test]
    fn test_arg_config_alias() {
        let m = cli().get_matches_from(vec!["prog", "--configuration", "config.toml"]);
        assert_eq!(
            get_arg_config(&m).map(|pb| format!("{}", pb.display())),
            Some("config.toml".into())
        );
    }

    /// Test passing the "config" argument via an enviroment variable.
    #[test]
    fn test_arg_config_env() -> std::io::Result<()> {
        check_environment(Some("config.toml"), None)
    }

    /// Verifies the child reads the environment values supplied by its parent.
    #[test]
    #[ignore = "needs a parent-controlled CLI environment"]
    fn child_checks_controlled_environment() {
        let expected_config = std::env::var("EXPECTED_CONFIG")
            .expect("the parent supplies the expected config value");
        let expected_verbose =
            std::env::var("EXPECTED_VERBOSE").expect("the parent supplies the expected verbosity");
        let matches = cli()
            .try_get_matches_from(["prog"])
            .expect("the controlled child environment should parse");
        let config = get_arg_config(&matches).map(|path| path.display().to_string());
        let expected_config = (expected_config != "<unset>").then_some(expected_config);
        assert_eq!(config, expected_config);
        assert_eq!(
            get_arg_verbose(&matches),
            expected_verbose
                .parse::<u8>()
                .expect("the parent supplies a numeric verbosity")
        );
    }

    /// Verifies that `--config` works globally, including after a subcommand.
    #[test]
    fn test_arg_config_global() {
        let m = cli().get_matches_from(vec!["prog", "open", "--config", "config.toml", "abc.csv"]);
        assert_eq!(
            get_arg_config(&m).map(|pb| format!("{}", pb.display())),
            Some("config.toml".into())
        );

        let m = cli().get_matches_from(vec!["prog", "open", "abc.csv", "--config", "config.toml"]);
        assert_eq!(
            get_arg_config(&m).map(|pb| format!("{}", pb.display())),
            Some("config.toml".into())
        );
    }

    // "filepaths" argument -----------

    /// Test the app-level "filepaths" argument.
    #[test]
    fn test_arg_filepaths_empty() {
        let m = cli().get_matches_from(vec!["prog"]);
        assert_eq!(get_arg_filepaths(&m), None);
    }

    /// Checks that several file paths are collected.
    #[test]
    fn test_arg_filepaths_multiple() {
        let m = cli().get_matches_from(vec!["prog", "a.csv", "b.csv"]);
        assert_eq!(
            get_arg_filepaths(&m),
            Some(vec![PathBuf::from("a.csv"), PathBuf::from("b.csv")])
        );
    }

    /// Checks that the verbosity is 0 by default.
    #[test]
    fn test_arg_verbose_none() -> std::io::Result<()> {
        check_environment(None, None)
    }

    /// Checks that repeated `-v` flags raise the verbosity.
    #[test]
    fn test_arg_verbose_multiple() {
        let m = cli().get_matches_from(vec!["prog", "-vvv"]);
        assert_eq!(get_arg_verbose(&m), 3);

        let m = cli().get_matches_from(vec!["prog", "-v", "-v"]);
        assert_eq!(get_arg_verbose(&m), 2);
    }

    /// Checks that the verbosity can be set with an environment variable.
    #[test]
    fn test_arg_verbose_env() -> std::io::Result<()> {
        check_environment(None, Some("1"))
    }
}
