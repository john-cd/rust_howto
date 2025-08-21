// [finish](https://github.com/john-cd/rust_howto/issues/1422)
#![allow(unused)]

mod cli;
mod conf;
mod links;
mod refdefs;

use clap::Parser;

fn main() -> anyhow::Result<()> {
    // Install a global tracing subscriber that listens for events
    // and filters based on the value of the RUST_LOG environment variable.
    tracing_subscriber::fmt::init();

    // Process command-line arguments to retrieve the directory to process:
    let args = cli::Args::parse();

    let conf = conf::Config::default();
    let scope = core_lib::Scope::default();

    for directory in &args.directories {
        let dir = directory.as_path().canonicalize()?;
        println!("Processing {}", dir.display());
        core_lib::walk_directory_and_process_files(&dir, &scope, check_links_in_file)?;
    }
    println!("DONE");
    Ok(())
}

fn check_links_in_file(filepath: &std::path::Path) -> anyhow::Result<()> {
    // Read a text file in memory, test if its contents should be processed, and if true, update its contents.
    core_lib::process_text_file(
        filepath,
        |s: &str| true, // TODO: check if the file contains any links.
        |s: &str| std::borrow::Cow::Borrowed(""), // FIXME: update the file.
    )?;
    Ok(())
}
