//! The command-line interface: arguments, subcommands and their parsing.
//!
//! TODO Reviewed ??-??-2026

use args::*;
use clap::ArgMatches;
use clap::Command;
use clap::builder::styling;
use clap::crate_version;

mod args;
mod cmd;
mod config;
mod opencmd;
mod querycmd;
mod testcmd;

pub(crate) use cmd::Cmd;

/// Define the command tree and arguments that the command-line
/// interface will accept.
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

    Command::new(env!("CARGO_CRATE_NAME")) // Or use: `crate_name!()`.
        .about("A demo CLI-based tool") // Sets the program's description for the short help (-h).
        .long_about("A tool that can open one or more files or query data.") // Sets the program's description for the long help (--help).
        // optional: .after_help("Longer explanation to appear after the options when displaying the help information from --help or -h")
        .help_expected(true) // Panic if help descriptions are omitted. This choice is propagated to all child subcommands.
        .flatten_help(true) // Flatten subcommand help into the current command's help.
        .version(crate_version!()) // Sets the version for the short version (-V) and help messages.
        // optional: .propagate_version(true) // Specifies to use the version of the current command for all subcommands.
        // optional: .author(clap::crate_authors!("\n")) // Set the authors. A custom help_template is needed for the author info to show up.
        .styles(styles)
        .args([arg_filepaths(), arg_config(), arg_verbose()])
        .infer_subcommands(true) // Allow partial matches of subcommand names and their aliases. For example, to match a subcommand named test, one could use t, te, tes, and test.
        // We define three subcommands.
        .subcommand(opencmd::cmd())
        .subcommand(querycmd::cmd())
        .subcommand(testcmd::cmd())
}

/// Parses the command line of the program, exiting on failure.
pub(crate) fn get_args() -> (config::Config, Vec<Cmd>) {
    // Parse [env::args_os], exiting on failure.
    let matches = cli().get_matches();
    parse_args(matches)
}

/// Converts the matches into a configuration and a list of commands.
fn parse_args(matches: ArgMatches) -> (config::Config, Vec<Cmd>) {
    let config = config::Config::builder()
        .set_verbose(get_arg_verbose(&matches))
        .set_config_file(get_arg_config(&matches));

    let mut commands: Vec<Cmd> = Vec::new();

    // Files passed directly as arguments.
    let mut opt_files = get_arg_filepaths(&matches);

    // Files passed after "open" command.
    opt_files = opt_files.map_or_else(
        // If None:
        || opencmd::get_args(&matches),
        // If Some, concatenate the filepaths vectors:
        |mut of| {
            if let Some(mut addtl_files) = opencmd::get_args(&matches) {
                of.append(&mut addtl_files);
            }
            Some(of)
        },
    );

    if let Some(files) = opt_files {
        commands.push(Cmd::Open(files));
    }

    let query = querycmd::get_args(&matches);
    if !query.is_empty() {
        commands.push(Cmd::Query(query));
    }

    if testcmd::is_present(&matches) {
        commands.push(Cmd::Test);
    }

    if commands.is_empty() {
        commands.push(Cmd::None);
    }

    (config.build(), commands)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that global arguments are parsed without a command.
    #[test]
    fn test_parse_args_global_arguments_no_cmd() {
        let m = cli().get_matches_from(vec!["foo", "-c", "config.toml", "-v"]);
        let (config, commands) = parse_args(m);
        assert_eq!(config.get_verbose(), 1);
        assert_eq!(config.get_config_file(), Some("config.toml".into()));
        for cmd in commands {
            assert_eq!(cmd, Cmd::None);
        }
    }

    /// Checks that the `test` command is parsed.
    #[test]
    fn test_parse_args_test_cmd() {
        let m = cli().get_matches_from(vec!["foo", "-c", "config.toml", "test"]);
        let (config, commands) = parse_args(m);
        assert_eq!(config.get_config_file(), Some("config.toml".into()));
        for cmd in commands {
            assert_eq!(cmd, Cmd::Test);
        }
    }

    /// Checks that the `query` command is parsed with its words.
    #[test]
    fn test_parse_args_query_cmd() {
        let m = cli().get_matches_from(vec!["foo", "query", "SELECT", "col", "FROM", "tbl"]);
        let (_config, commands) = parse_args(m);
        for cmd in commands {
            assert_eq!(
                cmd,
                Cmd::Query(
                    ["SELECT", "col", "FROM", "tbl"]
                        .iter()
                        .map(ToString::to_string)
                        .collect()
                )
            );
        }
    }

    /// Checks that the `open` command is parsed with its files.
    #[test]
    fn test_parse_args_open_cmd() {
        let m = cli().get_matches_from(vec!["foo", "-vv", "open", "a.csv"]);
        let (config, commands) = parse_args(m);
        assert_eq!(config.get_verbose(), 2);
        for cmd in commands {
            assert_eq!(cmd, Cmd::Open(vec!["a.csv".into()]));
        }
    }

    // Test that we can't pass both filepath arguments and the open
    // command. You can play with
    //  .subcommand_precedence_over_arg(true) // If true, prevent
    // subcommands from being consumed as an arguments value. and
    // .args_conflicts_with_subcommands(true) // If true, the use of an
    // argument prevents the use of subcommands, i.e. arguments can only
    // follow the final subcommand.  to change that behavior.
    /// Checks that file paths and the `open` command conflict.
    #[test]
    fn test_both_arg_filepaths_and_cmd_open() {
        let m = cli().get_matches_from(vec!["foo", "a.csv", "open", "b.csv"]);
        let (_config, commands) = parse_args(m);
        for cmd in commands {
            assert_eq!(
                cmd,
                Cmd::Open(vec!["a.csv".into(), "open".into(), "b.csv".into()])
            );
        }
    }

    // Passing multiple commands is not supported.
    /// Checks that only one command is accepted.
    #[test]
    fn test_multiple_cmds() {
        let m = cli().get_matches_from(vec![
            "foo", "query", "SELECT", "col", "FROM", "_file", "open", "a.csv",
        ]);
        let (_config, commands) = parse_args(m);
        assert_eq!(
            commands.first(),
            Some(&Cmd::Query(vec![
                "SELECT".into(),
                "col".into(),
                "FROM".into(),
                "_file".into(),
                "open".into(),
                "a.csv".into()
            ]))
        );
    }
}
