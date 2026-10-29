// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Main function that parses command line arguments and prints their values:

use std::path::PathBuf;

use clap::Arg;
use clap::Command;
use clap::value_parser;

/// Creates the command line interface for the application.
fn cli() -> Command {
    clap::Command::new("My Test Program")
        .bin_name("test_app")
        .version("0.1.0")
        .author("Hackerman Jones <hckrmnjones@hack.gov>")
        .about("Teaches argument parsing")
        // First possible argument: `--num` or `-n`.
        .arg(
            Arg::new("num")
                .short('n') // `-n` argument.
                .long("number") // `--number` long-form argument.
                .value_name("NUMBER") // Placeholder for the argument's value in the help message /
                // usage.
                .required(false)
                .help("Enter your favorite number"),
        )
        // Second possible argument: `--file` or `-f`:
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_parser(value_parser!(PathBuf))
                .help("Enter the path of a file"),
        )
    // We can also use the `arg!` macro:
    // `.arg(clap::arg!(-c --config <CONFIG> "Optionally sets a config file to
    // use"))`.
}

/// Main function that parses command line arguments and prints their values:
fn main() {
    let matches =
        cli().get_matches_from(["test_app", "-n", "42", "--file", "README.md"]);
    // In a real program, use the following to retrieve arguments from the
    // command line: `let matches = cli().get_matches();`.

    if let Some(num) = matches.get_one::<String>("num") {
        println!("Value for num: {num}");
    }

    if let Some(file_path) = matches.get_one::<PathBuf>("file") {
        println!("Value for file: {}", file_path.display());
    }
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the clap basic example through its entry point.
    #[test]
    fn test_clap_basic_example_entry_point() {
        main();
    }

    #[test]
    fn short_and_long_options_are_parsed() {
        let m = cli().get_matches_from([
            "test_app",
            "-n",
            "42",
            "--file",
            "README.md",
        ]);
        assert_eq!(m.get_one::<String>("num").map(String::as_str), Some("42"));
        assert_eq!(
            m.get_one::<PathBuf>("file"),
            Some(&PathBuf::from("README.md"))
        );
    }

    #[test]
    fn options_are_optional_and_unknown_flags_rejected() {
        let m = cli().get_matches_from(["test_app"]);
        assert!(m.get_one::<String>("num").is_none());
        assert!(m.get_one::<PathBuf>("file").is_none());
        assert!(cli().try_get_matches_from(["test_app", "--bogus"]).is_err());
        assert!(cli().try_get_matches_from(["test_app", "-n"]).is_err());
    }

    #[test]
    fn version_flag_prints_the_version() {
        let err = cli()
            .try_get_matches_from(["test_app", "--version"])
            .unwrap_err();
        assert_eq!(err.kind(), clap::error::ErrorKind::DisplayVersion);
        assert!(err.to_string().contains("0.1.0"));
    }
}
