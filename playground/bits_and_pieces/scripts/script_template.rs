#!/usr/bin/env -S cargo +nightly -Zscript

//!
//! TODO Reviewed ??-??-2026.

---cargo
[dependencies]
clap = { version = "4.5", features = ["derive"] }
---

//!

use clap::Parser;

/// Command-line options accepted by the script template.
#[derive(Parser, Debug)]
#[clap(version)]
struct Args {

    /// Optional path to the configuration file.
    #[clap(short, long, help = "Path to config")]
    config: Option<std::path::PathBuf>,
}

/// Parses the options and prints the optional configuration path.
fn main() {
    let args = Args::parse();
    println!("{args:?}");
}
