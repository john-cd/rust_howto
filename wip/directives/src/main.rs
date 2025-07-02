mod cli;
mod conf;
// TODO mod process;

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
        // TODO process::walk_directory(&dir, process::replace::replace_in_file)?;
    }
    println!("DONE");
    Ok(())
}
