// ANCHOR: example
use std::path::PathBuf;

use clap::value_parser;
use clap::Arg;
use clap::Command;

fn cli() -> Command {
    clap::Command::new("My Test Program")
        .bin_name("test_app")
        .version("0.1.0")
        .author("Hackerman Jones <hckrmnjones@hack.gov>")
        .about("Teaches argument parsing")
        // First possible argument: --num or -n
        .arg(Arg::new("num")
                .short('n')     // -n argument
                .long("number") // --number long-form argument
                .value_name("NUMBER") // placeholder for the argument's value in the help message / usage.
                .required(false)
                .help("Enter your favorite number"))
        // Second possible argument: --file or -f
        .arg(Arg::new("file")
                .short('f')
                .long("file")
                .value_parser(value_parser!(PathBuf))
                .help("Enter the path of a file"))
    // You can also use the arg! macro: .arg(clap::arg!(-c --config <CONFIG>
    // "Optionally sets a config file to use"))
}

fn main() {
    let matches =
        cli().get_matches_from(["test_app", "-n", "42", "--file", "README.md"]);
    // In a real program, use the following to retrieve arguments from the
    // command line: let matches = cli().get_matches();

    if let Some(num) = matches.get_one::<String>("num") {
        println!("Value for num: {num}");
    }

    if let Some(file_path) = matches.get_one::<PathBuf>("file") {
        println!("Value for file: {}", file_path.display());
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
