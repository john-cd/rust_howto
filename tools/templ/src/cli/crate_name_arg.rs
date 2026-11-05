//! Shared utilities functions
//!
//! TODO Reviewed ??-??-2026.

/// Defines a command-line argument for specifying crate names.
///
/// This function creates a `clap::Arg` that is used to collect one or more crate names from the
/// command line. The argument is required and allows multiple values to be specified.
pub(super) fn arg_crate_name() -> clap::Arg {
    clap::Arg::new("crate_name")
        .required(true)
        .value_name("CRATE_NAME") // placeholder for the argument's value in the help message / usage.
        .action(clap::ArgAction::Append)
        .help("Enter the crate name(s). Multiple crate names can be specified.")
}

/// Extracts the crate names from the command line arguments.
///
/// This function retrieves the values associated with the "`crate_name`" argument from the
/// `clap::ArgMatches`.
pub(super) fn get_cmd_arg_crate_name(m: &clap::ArgMatches) -> Vec<String> {
    m.get_many::<String>("crate_name")
        .unwrap_or_default()
        .map(std::convert::Into::into)
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    use clap::Command;

    use super::*;

    /// Checks that a single crate name is extracted.
    #[test]
    fn extracts_single_crate_name() {
        let matches = Command::new("test_cmd")
            .arg(arg_crate_name())
            .try_get_matches_from(["test_cmd", "serde"])
            .expect("arguments should parse");

        let names = get_cmd_arg_crate_name(&matches);
        assert_eq!(names, vec!["serde".to_owned()]);
    }

    /// Checks that multiple crate names are extracted in order.
    #[test]
    fn extracts_multiple_crate_names() {
        let matches = Command::new("test_cmd")
            .arg(arg_crate_name())
            .try_get_matches_from(["test_cmd", "serde", "tokio", "clap"])
            .expect("arguments should parse");

        let names = get_cmd_arg_crate_name(&matches);
        assert_eq!(
            names,
            vec!["serde".to_owned(), "tokio".to_owned(), "clap".to_owned()]
        );
    }

    /// Checks that missing crate name fails argument validation.
    #[test]
    fn missing_crate_name_fails() {
        let result = Command::new("test_cmd")
            .arg(arg_crate_name())
            .try_get_matches_from(["test_cmd"]);

        assert!(result.is_err(), "missing crate name should fail");
    }

    /// Checks that empty vector is returned when optional crate name argument is not provided.
    #[test]
    fn empty_vector_when_arg_not_present() {
        let matches = Command::new("test_cmd")
            .arg(arg_crate_name().required(false))
            .try_get_matches_from(["test_cmd"])
            .expect("arguments should parse");

        let names = get_cmd_arg_crate_name(&matches);
        assert!(names.is_empty());
    }
}
