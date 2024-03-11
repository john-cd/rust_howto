use clap::arg;
use clap::ArgMatches;
use clap::Command;

pub(super) fn cmd() -> Command {
    Command::new("query")
    // optional: .short_flag('q')
    .about("Query a data source")
    .arg(
        arg!(<query> ... "query to run")
            .required(true)
            .value_parser(clap::builder::NonEmptyStringValueParser::new())
            .trailing_var_arg(true) // everything that follows `query` should be captured
            .allow_negative_numbers(true),
    )
}
// See: https://docs.rs/clap/latest/clap/struct.Arg.html#method.trailing_var_arg

/// Returns ArgMatches for the `query`` subcommand. Returns empty Vec
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

    #[test]
    fn verify_query_cmd() {
        cmd().debug_assert(); // https://docs.rs/clap/latest/clap/struct.Command.html#method.debug_assert
    }
}
