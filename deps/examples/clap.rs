//! Useful links:
//! https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html
//! https://docs.rs/clap/latest/clap/_derive/_cookbook/index.html
//! https://github.com/clap-rs/clap/tree/master/examples

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use clap::Subcommand;

#[derive(Parser, Debug)]
// Reads the following attributes the from the package's `Cargo.toml`
#[command(author, version, about, long_about = None)]
// Displays Help if no arguments are provided
#[command(arg_required_else_help = true)]
pub struct Cli {
    // Positional argument example
    /// The pattern to look for (the doc comment appears in the help)
    pattern: String,
    // It could also be Option<...> for optional arguments

    // Named argument example
    /// The path to the file to look into
    #[arg(short, long)]
    path: Option<PathBuf>,

    // // Add-on functionality via https://docs.rs/clap-verbosity-flag/
    // // Adds the following flags through the entire program
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

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Read something
    Read {
        /// A boolean flag
        #[arg(short, long)]
        all: bool,
    },
    /// Say something
    Tell,
}

fn main() -> Result<()> {
    // `Clap` returns a populated Cli struct...
    let cli = Cli::parse();

    println!("Pattern: {:?}", cli.pattern);
    println!("Path: {:?}", cli.path);

    // // Use the `verbose` flag
    // env_logger::Builder::new()
    //     .filter_level(cli.verbose.log_level_filter())
    //     .init();

    // Check for the existence of subcommands
    match &cli.command {
        Some(Commands::Read { all }) => {
            if *all {
                println!("Read all...");
            } else {
                println!("Read just one...");
            }
        }
        Some(Commands::Tell) => { /*...*/ }
        None => {}
    }
    Ok(())
}
