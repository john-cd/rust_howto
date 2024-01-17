use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use clap::Subcommand;

// The struct declaring the desired command-line arguments and
// commands

// The `derive` feature flag is required (see Cargo.toml).
#[derive(Parser, Debug)]
// Reads the following attributes the from the package's `Cargo.toml`
// Alternatively, use #[command(name = "MyApp")] ...
#[command(author, version, about, long_about = None)]
// Displays Help if no arguments are provided
#[command(arg_required_else_help = true)]
pub struct Cli {
    // Positional argument example
    /// The pattern to look for (the doc comment appears in the help)
    pattern: Option<String>,

    /// Required argument example (with default value and validation)
    #[arg(default_value_t = 8080)]
    #[arg(value_parser = clap::value_parser!(u16).range(1..))]
    port: u16,

    // Named argument example: the path to the file to look into
    #[arg(short, long)]
    path: Option<PathBuf>,

    /// Count example: turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    // // Alternatively, use the
    // // [clap-verbosity-flag](https://docs.rs/clap-verbosity-flag/) crate:
    // // It adds the following flags through the entire program:
    // // -q silences output
    // // -v show warnings
    // // -vv show info
    // // -vvv show debug
    // // -vvvv show trace
    // //  By default, this will only report errors.
    // #[clap(flatten)]
    // verbose: clap_verbosity_flag::Verbosity,

    // Subcommands
    #[command(subcommand)]
    pub command: Option<Commands>,
}

// The subcommands
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Read something
    #[command(arg_required_else_help = true)]
    Read {
        /// A boolean flag
        #[arg(short, long)]
        all: bool,

        #[arg(required = true)]
        file: Vec<PathBuf>,
    },
    /// Say something
    Tell,
}

fn main() -> Result<()> {
    // `Clap` returns a Cli struct populated from `std::env::args_os()`...
    let cli = Cli::try_parse()?;
    // You also could use `parse()`

    // The argument values we got...
    println!("Path: {:?}", cli.path);

    // Use `unwrap_or` to set defaults for optional arguments
    // (or use `default_value_t`` as above).
    println!("Pattern: {:?}", cli.pattern.unwrap_or("".to_string()));

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    // // Alternatively, use the `verbose` flag, if configured above
    // env_logger::Builder::new()
    //     .filter_level(cli.verbose.log_level_filter())
    //     .init();

    // Check for the existence of subcommands
    match &cli.command {
        Some(Commands::Read { all, file }) => {
            if *all {
                println!("Read all...");
            } else {
                println!("Read just one...");
            }
            println!("{:?}", file);
        }
        Some(Commands::Tell) => {
            println!("{}", 42);
        }
        None => {}
    }
    Ok(())
}
