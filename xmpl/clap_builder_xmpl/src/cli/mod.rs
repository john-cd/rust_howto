use std::path::PathBuf;

use clap::builder::styling;
use clap::crate_version;
use clap::value_parser;
use clap::Arg;
use clap::ArgAction;
use clap::ArgMatches;
use clap::Command;
use clap::ValueHint;

use super::Cmd;

mod config;
mod opencmd;
mod querycmd;
mod testcmd;
// pub(crate) use config::Config;

fn cli() -> Command {
    let styles = styling::Styles::styled()
        // default .header(styling::Effects::BOLD | styling::Effects::UNDERLINE)
        // default .usage(styling::Effects::BOLD | styling::Effects::UNDERLINE)
        .literal(styling::AnsiColor::Blue.on_default() | styling::Effects::BOLD)
        .placeholder(styling::AnsiColor::Cyan.on_default());
    // Other styling options:
    // .error(styling::AnsiColor::Red.on_default() |
    // styling::Effects::BOLD) .valid(styling::AnsiColor::Green.
    // on_default()) .invalid(styling::AnsiColor::Yellow.
    // on_default());

    Command::new(env!("CARGO_CRATE_NAME")) // or use: crate_name!()
            .about("A CLI-based tool") // Sets the program's description for the short help (-h).
            .long_about("A tool that can open one or ore files and query data.") // Sets the program's description for the long help (--help).
            // optional: .after_help("Longer explanation to appear after the options when displaying the help information from --help or -h")
            .help_expected(true) // Panic if help descriptions are omitted. This choice is propagated to all child subcommands.
            .flatten_help(true) // Flatten subcommand help into the current command’s help
            .version(crate_version!()) // Sets the version for the short version (-V) and help messages.
            // optional: .propagate_version(true) // Specifies to use the version of the current command for all subcommands.
            // optional: .author(clap::crate_authors!("\n")) // Set the authors. A custom help_template is needed for author to show up.
            .styles(styles)
            .args( [arg_open(), arg_config(), arg_verbose()] )
            .infer_subcommands(true) // Allow partial matches of subcommand names and their aliases. For example, to match a subcommand named test, one could use t, te, tes, and test.
            .subcommand_precedence_over_arg(true) // Prevent subcommands from being consumed as an arguments value.
            //.args_conflicts_with_subcommands(true) // if true, the use of an argument prevents the use of subcommands.
            .subcommand(opencmd::cmd())
            .subcommand(querycmd::cmd())
            .subcommand(testcmd::cmd())
}

pub(crate) fn get_args() -> (config::Config, Cmd) {
    let matches = cli().get_matches();

    let config = config::Config::builder()
        .set_verbose(get_arg_verbose(&matches))
        .set_config_file(get_arg_config(&matches));

    let mut cmd = Cmd::default();

    let mut files = get_arg_open(&matches);
    if !files.is_empty() {
        cmd = Cmd::Open(files);
    }

    files = opencmd::get_args(&matches);
    if !files.is_empty() {
        cmd = Cmd::Open(files);
    }

    let query = querycmd::get_args(&matches);
    if !query.is_empty() {
        cmd = Cmd::Query(query);
    }

    if testcmd::is_present(&matches) {
        cmd = Cmd::Test;
    }

    (config.build(), cmd)
}

// (APP-LEVEL) ARGUMENTS -------------------------------

/// global "--config FILE" flag
///
/// Example: `cargo r -- --conf config.toml`
fn arg_config() -> Arg {
    Arg::new("config")
    .short('c')         // -c
    .long("config")     // --config
    .alias("conf")      // --conf (hidden alias - use `visible_alias` to show in the help message)
    // default .num_args(1)
    .value_name("FILE") // FILE, not <config>, is the placeholder for the argument's value in the help message / usage.
    .value_parser(value_parser!(PathBuf))   // Parse and validate the FILE value before storing it into ArgMatches as the given type.
    .value_hint(ValueHint::FilePath)        // Provide the shell a hint about how to complete this argument.
    .help("Provides a custom config file")
    // default .required(false) // The argument does not need to be present.
    // default .action(ArgAction::Set)
    .env("TOOL_CONFIG_FILE") // Read from the environment variable when the argument is not present.
    .global(true) // The --config flag can be passed to all child subcommands.
}

fn get_arg_config(matches: &ArgMatches) -> Option<PathBuf> {
    matches.get_one::<PathBuf>("config").cloned()
}

/// "filepath..." positional argument
///
/// Example: `cargo r -- file1.csv file2.csv`
fn arg_open() -> Arg {
    Arg::new("filepath")
        .value_parser(value_parser!(PathBuf))
        .help("File names or paths")
        .action(ArgAction::Append) // Allow passing multiple filepaths
}

fn get_arg_open(matches: &ArgMatches) -> Vec<PathBuf> {
    matches
        .get_many::<PathBuf>("filepath")
        .unwrap_or_default()
        .cloned()
        .collect::<Vec<_>>()
}

/// global `--verbose / -v` flag that can be added more than once
///
/// Example: `cargo r -- -vv`
fn arg_verbose() -> Arg {
    Arg::new("verbose")
        .short('v') // -v
        .long("verbose") // --verbose
        .help("Verbose output")
        .action(ArgAction::Count)
        // .default_value("1")
        .env("TOOL_VERBOSE") // example: NEXTCELL_VERBOSE=2 cargo run
        .global(true)
}

fn get_arg_verbose(matches: &ArgMatches) -> u8 {
    matches.get_count("verbose")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn verify_cmd() {
        cli().debug_assert();
    }
    // See: https://docs.rs/clap/latest/clap/struct.Command.html#method.debug_assert

    #[test]
    fn test_arg_config_short() {
        let m = cli().get_matches_from(vec!["prog", "-c", "config.toml"]);
        assert_eq!(
            get_arg_config(&m).map(|pb| format!("{}", pb.display())),
            Some("config.toml".into())
        );
    }

    #[test]
    fn test_arg_config_long() {
        let m = cli().get_matches_from(vec!["prog", "--config", "config.toml"]);
        assert_eq!(
            get_arg_config(&m).map(|pb| format!("{}", pb.display())),
            Some("config.toml".into())
        );
    }

    #[test]
    fn test_arg_config_alias() {
        let m = cli().get_matches_from(vec!["prog", "--conf", "config.toml"]);
        assert_eq!(
            get_arg_config(&m).map(|pb| format!("{}", pb.display())),
            Some("config.toml".into())
        );
    }

    #[test]
    fn test_arg_config_env() {
        let key = "NEXTCELL_CONFIG_FILE";
        std::env::set_var(key, "config.toml");
        let m = cli().try_get_matches_from(vec!["prog"]);
        // clean up the env variable
        std::env::remove_var(key);
        assert!(std::env::var(key).is_err());
        assert_eq!(
            get_arg_config(&m.unwrap()).map(|pb| format!("{}", pb.display())),
            Some("config.toml".into())
        );
    }

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

    #[test]
    fn test_arg_config_malformed() {
        let m = cli().try_get_matches_from(vec!["prog", "--config"]); // no value provided
        assert!(m.is_err());
    }

    #[test]
    fn test_arg_open_empty() {
        let m = cli().get_matches_from(vec!["prog"]);
        assert_eq!(get_arg_open(&m), Vec::<PathBuf>::new());
    }

    #[test]
    fn test_arg_open_multiple() {
        let m = cli().get_matches_from(vec!["prog", "a.csv", "b.csv"]);
        assert_eq!(
            get_arg_open(&m),
            vec![PathBuf::from("a.csv"), PathBuf::from("b.csv")]
        );
    }

    #[test]
    fn test_arg_verbose_none() {
        let m = cli().get_matches_from(vec!["prog"]);
        assert_eq!(get_arg_verbose(&m), 0);
    }

    #[test]
    fn test_arg_verbose_multiple() {
        let m = cli().get_matches_from(vec!["prog", "-vvv"]);
        assert_eq!(get_arg_verbose(&m), 3);

        let m = cli().get_matches_from(vec!["prog", "-v", "-v"]);
        assert_eq!(get_arg_verbose(&m), 2);
    }

    #[test]
    fn test_arg_verbose_env() {
        let key = "NEXTCELL_VERBOSE";
        std::env::set_var(key, "1");
        let m = cli().try_get_matches_from(vec!["prog"]);
        // clean up the env variable
        std::env::remove_var(key);
        assert!(std::env::var(key).is_err());
        assert_eq!(get_arg_verbose(&m.unwrap()), 1);
    }

    #[test]
    fn test_cmd_open() {
        let m = cli().get_matches_from(vec!["foo", "open", "a.csv", "b.csv"]);
        assert_eq!(
            opencmd::get_args(&m),
            vec![PathBuf::from("a.csv"), PathBuf::from("b.csv")]
        );
    }

    #[test]
    fn test_cmd_query() {
        let m = cli().get_matches_from(vec![
            "foo", "query", "SELECT", "col", "FROM", "tbl", "open", "data.csv",
        ]);
        assert_eq!(
            querycmd::get_args(&m),
            vec!["SELECT", "col", "FROM", "tbl"]
        );
        assert_eq!(opencmd::get_args(&m), vec![PathBuf::from("a.csv")]);
    }

    #[test]
    fn test_cmd_test() {
        let m = cli().get_matches_from(vec!["foo", "test"]);
        assert!(testcmd::is_present(&m));
    }

    #[test]
    fn test_cmd_test_infer() {
        let m = cli().get_matches_from(vec!["foo", "te"]);
        assert!(testcmd::is_present(&m));
    }
}
