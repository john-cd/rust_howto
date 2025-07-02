#![allow(dead_code)]
//! Convert any naked URLs and inline links in Markdown files within a folder into reference-style links.
//!
//! - Convert inline links e.g. [...](http(s)://...) into reference-style links: [...][...] [...]: http://...
//! - Convert http(s)://... naked links into reference-style links.
//! - Skip URLs between ``` and ``` and within hidden sections: <div class="hidden">...</div>
//! - Ignore URLs within reference-style link labels: [https://...][...]
//! - Do not convert links to GitHub issues e.g. https://github.com/john-cd/rust_howto/issues links.
//! - Move refdefs into a central file (and sort them).

mod cli;
mod process;

use clap::Parser;

fn main() -> anyhow::Result<()> {
    // Install a global tracing subscriber that listens for events
    // and filters based on the value of the RUST_LOG environment variable.
    tracing_subscriber::fmt::init();

    // Process command-line arguments to retrieve the directory to process:
    let args = cli::Args::parse();
    for directory in &args.directories {
        let dir = directory.as_path().canonicalize()?;
        println!("Processing {}", dir.display());
        process::walk_directory(&dir, process::replace::replace_in_file)?;
    }
    println!("DONE");
    Ok(())
}

// TODO finish
