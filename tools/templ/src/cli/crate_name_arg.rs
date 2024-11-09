//! Shared utilities functions

pub(super) fn arg_crate_name() -> clap::Arg {
    clap::Arg::new("crate_name")
                .required(true)
                .value_name("CRATE_NAME") // placeholder for the argument's value in the help message / usage.
                .action(clap::ArgAction::Append)
                .help("Enter the crate name(s)")
}

pub(super) fn get_cmd_arg_crate_name(m: &clap::ArgMatches) -> Vec<String> {
    m.get_many::<String>("crate_name")
        .unwrap_or_default()
        .map(|v| v.into())
        .collect::<Vec<String>>()
}
