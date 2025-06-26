use anyhow::Result;
use clap::Parser;

mod cli;
mod replace;
mod walk;

fn main() -> Result<()> {
    // Install a global tracing subscriber that listens for events and filters based on the value of the RUST_LOG environment variable.
    tracing_subscriber::fmt::init();

    let args = cli::Args::parse();
    walk::walk_directory(args.directory.as_path())?;
    Ok(())
}
