//! Runs one of the `tar` examples, chosen on the command line.
//!
//!
//! A tar file is an archive: it bundles many files and directories into one
//! file, keeping paths and permissions, but it does not compress. It is usually
//! combined with gzip, giving `.tar.gz`, to ship a whole directory.
//!
//! TODO Reviewed ??-??-2026.

use std::path::PathBuf;

use clap::Parser;
use clap::Subcommand;

/// This function creates a gzip-compressed tar archive from a directory.
mod tar_compress;
/// Unpacking a gzip-compressed tar archive with `flate2` and `tar`.
mod tar_decompress;
/// Extract regular files from a gzipped tar archive after removing a shared path prefix.
mod tar_strip_prefix;

/// Command-line arguments.
#[derive(Parser)]
struct Cli {
    /// The example to run.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// The examples that can be run.
#[derive(Subcommand)]
enum Commands {
    /// Runs the `tar_compress` example.
    #[command(name = "tar_compress")]
    Compress {
        /// A new directory for the sample inputs and archive.
        workspace: PathBuf,
    },

    /// Runs the `tar_decompress` example.
    #[command(name = "tar_decompress")]
    Decompress {
        /// The workspace created by the compression example.
        workspace: PathBuf,
    },

    /// Runs the `tar_strip_prefix` example.
    #[command(name = "tar_strip_prefix")]
    StripPrefix,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Compress { workspace } => {
                tar_compress::run(&workspace)?;
            }
            Commands::Decompress { workspace } => {
                tar_decompress::run(&workspace)?;
            }
            Commands::StripPrefix => {
                tar_strip_prefix::run()?;
            }
        }
    }
    Ok(())
}
