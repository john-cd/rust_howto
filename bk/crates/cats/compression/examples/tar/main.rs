//! Runs one of the `tar` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026

use clap::Parser;
use clap::Subcommand;

#[cfg(test)]
static CWD_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

mod tar_compress;
mod tar_decompress;
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
    Compress,
    /// Runs the `tar_decompress` example.
    #[command(name = "tar_decompress")]
    Decompress,
    /// Runs the `tar_strip_prefix` example.
    #[command(name = "tar_strip_prefix")]
    StripPrefix,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Compress => {
                tar_compress::run()?;
            }
            Commands::Decompress => {
                tar_decompress::run()?;
            }
            Commands::StripPrefix => {
                tar_strip_prefix::run()?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::fs::{self};
    use std::time::SystemTime;

    use flate2::Compression;
    use flate2::write::GzEncoder;

    use super::*;

    #[test]
    fn test_compress() -> anyhow::Result<()> {
        let _cwd_lock = CWD_LOCK.lock().unwrap();
        tar_compress::run()?;
        Ok(())
    }

    #[test]
    fn test_decompress() -> anyhow::Result<()> {
        let _cwd_lock = CWD_LOCK.lock().unwrap();
        tar_compress::run()?;
        tar_decompress::run()?;
        Ok(())
    }

    #[test]
    fn test_strip_prefix() -> anyhow::Result<()> {
        let work_dir = std::env::temp_dir().join(format!(
            "rust_howto_tar_strip_prefix_{}_{:?}_{}_{}",
            std::process::id(),
            std::thread::current().id(),
            std::path::Path::new(file!())
                .file_name()
                .unwrap()
                .to_string_lossy(),
            SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_nanos()
        ));
        if work_dir.exists() {
            fs::remove_dir_all(&work_dir)?;
        }
        fs::create_dir_all(work_dir.join("bundle/logs"))?;
        let source_file = work_dir.join("bundle/logs/example.txt");
        fs::write(&source_file, b"hello world")?;
        fs::create_dir_all(work_dir.join("temp"))?;

        let archive_path = work_dir.join("temp/archive.tar.gz");
        let archive_file = File::create(&archive_path)?;
        let enc = GzEncoder::new(archive_file, Compression::default());
        let mut tar_builder = tar::Builder::new(enc);
        tar_builder
            .append_path_with_name(&source_file, "bundle/logs/example.txt")?;
        let enc = tar_builder.into_inner()?;
        enc.finish()?;

        let _cwd_lock = CWD_LOCK.lock().unwrap();
        let original_dir = std::env::current_dir()?;
        std::env::set_current_dir(&work_dir)?;
        let result = tar_strip_prefix::run();
        std::env::set_current_dir(original_dir)?;

        result?;
        assert!(work_dir.join("example.txt").exists());
        fs::remove_dir_all(&work_dir)?;
        Ok(())
    }
}
