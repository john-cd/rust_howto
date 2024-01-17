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
// Reads the following attributes from the package's `Cargo.toml`
#[command(author, version, about, long_about = None)]
// Displays the help, if no arguments are provided
#[command(arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    RefDefs {
        // The path to the file to write
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
    Test,
    Debug,
}

pub fn parse_arguments() -> Cli {
    Cli::parse()
}
