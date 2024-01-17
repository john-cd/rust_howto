//! Command-line argument parser
//!
//! Useful links:
//! https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html
//! https://docs.rs/clap/latest/clap/_derive/_cookbook/index.html
//! https://github.com/clap-rs/clap/tree/master/examples

use std::path::PathBuf;

use clap::Args;
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
    /// Write existing reference definitions to a file
    RefDefs(PathArgs),

    /// Write all existing links to a file
    Links(PathArgs),

    /// Write all existing inline links to a file
    InlineLinks(PathArgs),

    /// Parse the entire Markdown code as events and print them.
    Debug(PathArgs),

    Test,
}

#[derive(Args, Debug)]
pub struct PathArgs {
    // The path to the file to write (optional)
    #[arg(short, long)]
    pub path: Option<PathBuf>,
}

pub fn parse_arguments() -> Cli {
    Cli::parse()
}
