use std::ffi::OsString;
use std::path::PathBuf;

use clap::Arg;
use clap::arg;
use clap::command;
use clap::value_parser;

use super::CommandLineArgs;

/// Define the command-line arguments.
fn cmd() -> clap::Command {
    command!()
        // Optional run mode (a.k.a. "environment"), which can be "production" or "development" and defaults to the latter.
        .arg(
            Arg::new("run_mode")
                .short('e')
                .long("env")
                .required(false)
                .value_parser(["production", "development"])
                .default_value("development"),
        )
        // Optional custom config file.
        .arg(
            arg!(
                -c --config <FILE> "Sets a custom config file"
            )
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        // Optional port number.
        .arg(
            arg!(-p --port <PORT> "Sets the service's port")
                .required(false)
                .value_parser(value_parser!(u16)),
        )
}

/// Parses command-line arguments.
///
/// This function parses command-line arguments using the `clap` crate.
/// - Run mode (with default value),
/// - Optional custom config file path,
/// - Optional port number override.
///
/// It can also be used for testing by passing in a custom set of arguments.
pub fn parse_command_line_args<I, T>(opt_args: Option<I>) -> CommandLineArgs
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    // Get command-line arguments from `std::env::args_os`,
    // unless they are passed manually in `opt_args`
    // (e.g. for testing purposes).
    let matches = if let Some(args) = opt_args {
        cmd().get_matches_from(args)
    } else {
        cmd().get_matches()
    };

    let run_mode: String = matches
        .get_one::<String>("run_mode")
        .expect("run_mode has a default value.")
        .clone();

    // Get the provided path to the custom config file, if any.
    let custom_config_file_path = matches.get_one::<PathBuf>("config").cloned();

    let port: Option<u16> = matches.get_one("port").copied();

    CommandLineArgs {
        run_mode,
        custom_config_file_path,
        port,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cmd() {
        cmd().debug_assert();
    }
}
