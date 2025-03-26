//! Shared utilities functions

/// Defines a command-line argument for specifying crate names.
///
/// This function creates a `clap::Arg` that is used to collect one or more crate names from the command line.
/// The argument is required and allows multiple values to be specified.
pub(super) fn arg_crate_name() -> clap::Arg {
    clap::Arg::new("crate_name")
        .required(true)
        .value_name("CRATE_NAME") // placeholder for the argument's value in the help message / usage.
        .action(clap::ArgAction::Append)
        .help("Enter the crate name(s). Multiple crate names can be specified.")
}

/// Extracts the crate names from the command line arguments.
///
/// This function retrieves the values associated with the "crate_name" argument from the `clap::ArgMatches`.
pub(super) fn get_cmd_arg_crate_name(m: &clap::ArgMatches) -> Vec<String> {
    m.get_many::<String>("crate_name")
        .unwrap_or_default()
        .map(|v| v.into())
        .collect::<Vec<String>>()
}
