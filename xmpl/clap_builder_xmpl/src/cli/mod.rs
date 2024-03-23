#![allow(unused_imports)]

use std::path::PathBuf;

use args::*;
use clap::builder::styling;
use clap::crate_version;
use clap::ArgMatches;
use clap::Command;

mod args;
mod cmd;
mod config;
mod opencmd;
mod querycmd;
mod testcmd;
pub(crate) use cmd::Cmd;
pub(crate) use config::Config;

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

    Command::new(env!("CARGO_CRATE_NAME")) // or use: crate_name!()
            .about("A demo CLI-based tool") // Sets the program's description for the short help (-h).
            .long_about("A tool that can open one or more files or query data.") // Sets the program's description for the long help (--help).
            // optional: .after_help("Longer explanation to appear after the options when displaying the help information from --help or -h")
            .help_expected(true) // Panic if help descriptions are omitted. This choice is propagated to all child subcommands.
            .flatten_help(true) // Flatten subcommand help into the current command’s help
            .version(crate_version!()) // Sets the version for the short version (-V) and help messages.
            // optional: .propagate_version(true) // Specifies to use the version of the current command for all subcommands.
            // optional: .author(clap::crate_authors!("\n")) // Set the authors. A custom help_template is needed for the author info to show up.
            .styles(styles)
            .args( [arg_filepaths(), arg_config(), arg_verbose()] )
            .infer_subcommands(true) // Allow partial matches of subcommand names and their aliases. For example, to match a subcommand named test, one could use t, te, tes, and test.
            // We define three subcommands.
            .subcommand(opencmd::cmd())
            .subcommand(querycmd::cmd())
            .subcommand(testcmd::cmd())
}

pub(crate) fn get_args() -> (config::Config, Vec<Cmd>) {
    // Parse [env::args_os], exiting on failure.
    let matches = cli().get_matches();
    parse_args(matches)
}

fn parse_args(matches: ArgMatches) -> (config::Config, Vec<Cmd>) {
    let config = config::Config::builder()
        .set_verbose(get_arg_verbose(&matches))
        .set_config_file(get_arg_config(&matches));

    let mut cmds: Vec<Cmd> = Vec::new();

    // files passed directly as arguments
    let mut opt_files = get_arg_filepaths(&matches);

    // files passed after "open" command
    opt_files = opt_files.map_or_else(
        // If None
        || opencmd::get_args(&matches),
        // If Some, concatenate the filepaths vectors
        |mut of| {
            if let Some(mut addtl_files) = opencmd::get_args(&matches) {
                of.append(&mut addtl_files);
            };
            Some(of)
        },
    );

    if let Some(files) = opt_files {
        cmds.push(Cmd::Open(files));
    }

    let query = querycmd::get_args(&matches);
    if !query.is_empty() {
        cmds.push(Cmd::Query(query));
    }

    if testcmd::is_present(&matches) {
        cmds.push(Cmd::Test);
    }

    if cmds.is_empty() {
        cmds.push(Cmd::None);
    }

    (config.build(), cmds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_args_global_arguments_no_cmd() {
        let m = cli().get_matches_from(vec!["foo", "-c", "config.toml", "-v"]);
        let (config, cmds) = parse_args(m);
        assert_eq!(config.get_verbose(), 1);
        assert_eq!(config.get_config_file(), Some("config.toml".into()));
        for cmd in cmds {
            assert_eq!(cmd, Cmd::None);
        }
    }

    #[test]
    fn test_parse_args_test_cmd() {
        let m =
            cli().get_matches_from(vec!["foo", "-c", "config.toml", "test"]);
        let (config, cmds) = parse_args(m);
        assert_eq!(config.get_config_file(), Some("config.toml".into()));
        for cmd in cmds {
            assert_eq!(cmd, Cmd::Test);
        }
    }

    #[test]
    fn test_parse_args_query_cmd() {
        let m = cli().get_matches_from(vec![
            "foo", "query", "SELECT", "col", "FROM", "tbl",
        ]);
        let (_config, cmds) = parse_args(m);
        for cmd in cmds {
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

    #[test]
    fn test_parse_args_open_cmd() {
        let m = cli().get_matches_from(vec!["foo", "-vv", "open", "a.csv"]);
        let (config, cmds) = parse_args(m);
        assert_eq!(config.get_verbose(), 2);
        for cmd in cmds {
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
    #[test]
    fn test_both_arg_filepaths_and_cmd_open() {
        let m = cli().get_matches_from(vec!["foo", "a.csv", "open", "b.csv"]);
        let (_config, cmds) = parse_args(m);
        for cmd in cmds {
            assert_eq!(
                cmd,
                Cmd::Open(vec!["a.csv".into(), "open".into(), "b.csv".into()])
            );
        }
    }

    // Passing multiple commands is not supported.
    #[test]
    fn test_multiple_cmds() {
        let m = cli().get_matches_from(vec![
            "foo", "query", "SELECT", "col", "FROM", "_file", "open", "a.csv",
        ]);
        let (_config, cmds) = parse_args(m);
        assert_eq!(
            cmds.first(),
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
