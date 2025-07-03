use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)] // Read from `Cargo.toml`
pub struct Args {
    /// Directories to process.
    pub directories: Vec<std::path::PathBuf>,
}
