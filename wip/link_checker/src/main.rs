// TODO
#![allow(unused)]

mod cli;
mod conf;
mod links;
mod refdefs;

use clap::Parser;
// use conf::Config;
use core_lib::walk_directory_and_process_files;

fn main() -> anyhow::Result<()> {
    // Install a global tracing subscriber that listens for events
    // and filters based on the value of the RUST_LOG environment variable.
    tracing_subscriber::fmt::init();

    // Process command-line arguments to retrieve the directory to process:
    let args = cli::Args::parse();
    for directory in &args.directories {
        let dir = directory.as_path().canonicalize()?;
        println!("Processing {}", dir.display());
        // TODO walk_directory_and_process_files(&dir, check_links_in_file)?;
    }
    println!("DONE");
    Ok(())
}
