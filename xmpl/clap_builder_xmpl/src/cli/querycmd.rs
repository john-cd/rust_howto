use clap::ArgMatches;
use clap::Command;
use clap::arg;

pub(super) fn cmd() -> Command {
    Command::new("query")
        // optional: .short_flag('q')
        .about("Query a data source")
        .arg(
            arg!(<query> ... "query to run")
                .required(true)
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                // .trailing_var_arg(true) // everything that follows `query` should be captured
                .allow_negative_numbers(true),
        )
}
// See: https://docs.rs/clap/latest/clap/struct.Arg.html#method.trailing_var_arg

/// Returns `ArgMatches` for the `query` subcommand. Returns empty Vec
/// if the subcommand wasn’t present at runtime.
pub(super) fn get_args(matches: &ArgMatches) -> Vec<String> {
    if let Some(sub_m) = matches.subcommand_matches("query") {
        sub_m
            .get_many::<String>("query")
            .unwrap_or_default()
            .cloned()
            .collect()
    } else {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::cli;

    #[test]
    fn verify_query_cmd() {
        cmd().debug_assert(); // https://docs.rs/clap/latest/clap/struct.Command.html#method.debug_assert
    }

    #[test]
    fn test_cmd_query() {
        let m = cli().get_matches_from(vec!["foo", "query", "SELECT", "col", "FROM", "tbl"]);
        assert_eq!(get_args(&m), vec!["SELECT", "col", "FROM", "tbl"]);
    }

    #[test]
    fn test_cmd_query_none() {
        let m = cli().try_get_matches_from(vec!["foo", "query"]);
        assert!(m.is_err());

        // empty string
        let m = cli().try_get_matches_from(vec!["foo", ""]);
        assert!(m.is_err());
    }

    // Verify that we don't get confused by negative numbers (that look
    // like flags).
    #[test]
    fn test_cmd_query_neg_numbers() {
        let m = cli().get_matches_from(vec!["foo", "query", "SELECT", "-1", "FROM", "tbl"]);
        assert_eq!(get_args(&m), vec!["SELECT", "-1", "FROM", "tbl"]);
    }
}
