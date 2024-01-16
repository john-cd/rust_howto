//! Command-line argument parser
//!
//! Useful links:
//! https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html
//! https://docs.rs/clap/latest/clap/_derive/_cookbook/index.html
//! https://github.com/clap-rs/clap/tree/master/examples

use std::path::PathBuf;

use clap::Parser;
use clap::Subcommand;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)] // Reads from `Cargo.toml`
#[command(arg_required_else_help = true)] // displays Help if no arguments are provided
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    // // Positional argument example
    // /// The pattern to look for
    // pattern: Option<String>,

    // Named argumente example
    // The path to the file to read
    // #[arg(short, long)]
    // path: Option<PathBuf>,

    // Add the following flags through the entire program
    // -q silences output
    // -v show warnings
    // -vv show info
    // -vvv show debug
    // -vvvv show trace
    //  By default, this will only report errors.
    //#[clap(flatten)]
    // verbose: clap_verbosity_flag::Verbosity, // https://docs.rs/clap-verbosity-flag/2.0.1/clap_verbosity_flag/
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Parse,
}

pub fn parse_arguments() -> Cli {
    Cli::parse()
}
// Then check for the existence of subcommands, and if found use their
// matches just as you would the top level cmd
// e.g.
// match &cli.command {
//     Some(Commands::Test { list }) => {
//         if *list {
//                println!("Printing lists...");
//         } else {
//              println!("Not printing lists...");
//         }
//     }
//     None => {}
// }
