#!/usr/bin/env -S cargo +nightly -Zscript
---cargo
[package]
edition = "2024"

[dependencies]
clap = { version = "4.6.7", features = ["derive"] }
---

// TODO Reviewed ??-??-2026.

use std::path::PathBuf;

use clap::Parser;

/// Prints the configuration path selected by the caller.
#[derive(Parser)]
struct Args {
    /// Path to the configuration file.
    #[arg(short, long, default_value = "settings.toml")]
    config: PathBuf,
}

/// Runs the example demonstrated by this module.
fn main() {
    let args = Args::parse();
    println!("{}", args.config.display());
}
