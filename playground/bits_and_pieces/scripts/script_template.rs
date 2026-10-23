#!/usr/bin/env -S cargo +nightly -Zscript
---cargo
[dependencies]
clap = { version = "4.5", features = ["derive"] }
---

//! Template for a runnable Rust script that parses command-line arguments.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;

/// Command-line options accepted by the script template.
#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    /// Optional path to the configuration file.
    #[clap(short, long, help = "Path to config")]
    config: Option<std::path::PathBuf>,
}

/// Parses the command-line options and prints them.
fn main() {
    let args = Args::parse();
    println!("{args:?}");
}
