use clap::ArgMatches;
use clap::Command;

/// Define a `test` command.
pub(super) fn cmd() -> Command {
    Command::new("test").about("Test command").hide(true) // hidden from the help message.
}

pub(super) fn is_present(matches: &ArgMatches) -> bool {
    matches.subcommand_name() == Some("test")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test_cmd() {
        cmd().debug_assert(); // https://docs.rs/clap/latest/clap/struct.Command.html#method.debug_assert
    }
}
