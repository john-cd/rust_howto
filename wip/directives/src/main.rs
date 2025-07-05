// TODO
#![allow(dead_code)]
#![allow(unused)]

mod category_links_badges;
mod cli;
mod common;
mod conf;
mod crate_blocks;
mod crate_links_badges;
mod examples;
mod wikilinks;

use clap::Parser;
use core_lib::walk_directory_and_process_files;
// use conf::Config;
use crate_links_badges::*;

fn main() -> anyhow::Result<()> {
    // Install a global tracing subscriber that listens for events
    // and filters based on the value of the RUST_LOG environment variable.
    tracing_subscriber::fmt::init();

    // Process command-line arguments to retrieve the directory to process:
    let args = cli::Args::parse();
    let scope = core_lib::Scope::default();

    for directory in &args.directories {
        let dir = directory.as_path().canonicalize()?;
        println!("Processing {}", dir.display());

        // TODO
        walk_directory_and_process_files(&dir, &scope, process_crate_badge_directives_in_file)?;
    }
    println!("DONE");
    Ok(())
}
