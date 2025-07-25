#![allow(unused)]

mod cli;

use clap::Parser;

fn main() -> anyhow::Result<()> {
    // Install a global tracing subscriber that listens for events
    // and filters based on the value of the RUST_LOG environment variable.
    tracing_subscriber::fmt::init();

    // Process command-line arguments to retrieve the directory to process:
    let args = cli::Args::parse();
    let scope = core_lib::Scope::default();

    // [finish](https://github.com/john-cd/rust_howto/issues/1423)
    println!("DONE");
    Ok(())
}
