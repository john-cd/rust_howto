use clap::ArgMatches;
use clap::Command;

/// Define a hidden `test` command that takes no arguments.
pub(super) fn cmd() -> Command {
    Command::new("test").about("Test command").hide(true)
    // not displayed in the help message.
}

pub(super) fn is_present(matches: &ArgMatches) -> bool {
    matches.subcommand_name() == Some("test")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::cli;

    #[test]
    fn verify_test_cmd() {
        cmd().debug_assert(); // <https://docs.rs/clap/latest/clap/struct.Command.html#method.debug_assert>
    }

    #[test]
    fn test_cmd_test() {
        let m = cli().get_matches_from(vec!["foo", "test"]);
        assert!(is_present(&m));
    }

    // Test partial matches of subcommand names and their aliases.
    #[test]
    fn test_cmd_test_infer() {
        let m = cli().get_matches_from(vec!["foo", "te"]);
        assert!(is_present(&m));
    }
}
