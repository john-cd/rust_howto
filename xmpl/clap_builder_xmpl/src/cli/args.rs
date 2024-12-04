use std::path::PathBuf;

use clap::Arg;
use clap::ArgAction;
use clap::ArgMatches;
use clap::ValueHint;
use clap::value_parser;

// (APP-LEVEL) ARGUMENTS -------------------------------

/// global "--config FILE" flag
///
/// Example: `cargo r -- --conf config.toml`
pub(super) fn arg_config() -> Arg {
    Arg::new("config")
    .short('c')         // -c
    .long("config")     // --config
    .alias("configuration")      // --configuration (hidden alias - use `visible_alias` to show in the help message)
    // default .num_args(1)
    .value_name("FILE") // FILE, not <config>, is the placeholder for the argument's value in the help message / usage.
    .value_parser(value_parser!(PathBuf))   // Parse and validate the FILE value before storing it into ArgMatches as the given type.
    .value_hint(ValueHint::FilePath)        // Provide the shell a hint about how to complete this argument.
    .help("Provides a custom config file")
    // default .required(false) // The argument does not need to be present.
    // default .action(ArgAction::Set)
    .env("TOOL_CONFIG_FILE") // Read from the environment variable when the argument is not present.
    .global(true) // The --config flag can be passed to all child
    // subcommands.
}

pub(super) fn get_arg_config(matches: &ArgMatches) -> Option<PathBuf> {
    matches.get_one::<PathBuf>("config").cloned()
}

/// "filepath..." positional argument
///
/// Example: `cargo r -- file1.csv file2.csv`
pub(super) fn arg_filepaths() -> Arg {
    Arg::new("filepaths")
        .value_parser(value_parser!(PathBuf))
        .help("File names or paths")
        .action(ArgAction::Append) // Allow passing multiple filepaths
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

/// Global `--verbose / -v` flag that can be added more than once
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
    use super::*;
    use crate::cli::cli;

    /// Verify that the global argument "config" requires a value.
    #[test]
    fn test_arg_config_malformed() {
        let m = cli().try_get_matches_from(vec!["prog", "--config"]); // no value provided
        assert!(m.is_err());
    }

    /// Test the global argument "config"
    #[test]
    fn test_arg_config_short() {
        let m = cli().get_matches_from(vec!["prog", "-c", "config.toml"]);
        assert_eq!(
            get_arg_config(&m).map(|pb| format!("{}", pb.display())),
            Some("config.toml".into())
        );
    }

    /// Test the global argument "config", long form
    #[test]
    fn test_arg_config_long() {
        let m = cli().get_matches_from(vec!["prog", "--config", "config.toml"]);
        assert_eq!(
            get_arg_config(&m).map(|pb| format!("{}", pb.display())),
            Some("config.toml".into())
        );
    }

    /// Test the global argument "config", alias form
    #[test]
    fn test_arg_config_alias() {
        let m = cli().get_matches_from(vec![
            "prog",
            "--configuration",
            "config.toml",
        ]);
        assert_eq!(
            get_arg_config(&m).map(|pb| format!("{}", pb.display())),
            Some("config.toml".into())
        );
    }

    /// Test passing the "config" argument via an enviroment variable
    #[test]
    fn test_arg_config_env() {
        let key = "TOOL_CONFIG_FILE";
        std::env::set_var(key, "config.toml");
        let m = cli().try_get_matches_from(vec!["prog"]);
        // Clean up the env variable
        std::env::remove_var(key);
        assert!(std::env::var(key).is_err());
        assert_eq!(
            get_arg_config(&m.unwrap()).map(|pb| format!("{}", pb.display())),
            Some("config.toml".into())
        );
    }

    /// Verify that "--config" can be used globally
    /// e.g. after a subcommand
    #[test]
    fn test_arg_config_global() {
        let m = cli().get_matches_from(vec![
            "prog",
            "open",
            "--config",
            "config.toml",
            "abc.csv",
        ]);
        assert_eq!(
            get_arg_config(&m).map(|pb| format!("{}", pb.display())),
            Some("config.toml".into())
        );

        let m = cli().get_matches_from(vec![
            "prog",
            "open",
            "abc.csv",
            "--config",
            "config.toml",
        ]);
        assert_eq!(
            get_arg_config(&m).map(|pb| format!("{}", pb.display())),
            Some("config.toml".into())
        );
    }

    // "filepaths" argument -----------

    /// Test the app-level "filepaths" argument
    #[test]
    fn test_arg_filepaths_empty() {
        let m = cli().get_matches_from(vec!["prog"]);
        assert_eq!(get_arg_filepaths(&m), None);
    }

    #[test]
    fn test_arg_filepaths_multiple() {
        let m = cli().get_matches_from(vec!["prog", "a.csv", "b.csv"]);
        assert_eq!(
            get_arg_filepaths(&m),
            Some(vec![PathBuf::from("a.csv"), PathBuf::from("b.csv")])
        );
    }

    // TODO P1 fix test
    // #[test]
    // fn test_arg_verbose_none() {
    //     let m = cli().get_matches_from(vec!["prog"]);
    //     assert_eq!(get_arg_verbose(&m), 0);
    // }

    #[test]
    fn test_arg_verbose_multiple() {
        let m = cli().get_matches_from(vec!["prog", "-vvv"]);
        assert_eq!(get_arg_verbose(&m), 3);

        let m = cli().get_matches_from(vec!["prog", "-v", "-v"]);
        assert_eq!(get_arg_verbose(&m), 2);
    }

    #[test]
    fn test_arg_verbose_env() {
        let key = "TOOL_VERBOSE";
        std::env::set_var(key, "1");
        let m = cli().try_get_matches_from(vec!["prog"]);
        // clean up the env variable
        std::env::remove_var(key);
        assert!(std::env::var(key).is_err());
        assert_eq!(get_arg_verbose(&m.unwrap()), 1);
    }
}
