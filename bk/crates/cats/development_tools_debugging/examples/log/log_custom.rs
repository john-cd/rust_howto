// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Configures and initializes the logging system using log4rs.
//!
//! The `log` crate is a facade: libraries call `info!`, `warn!` and so on, and
//! the application picks the logger that decides where the messages go.
//! Implementing the `Log` trait gives a custom destination, such as a file or a
//! service.

use std::path::Path;

use anyhow::Result;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::Appender;
use log4rs::config::Config;
use log4rs::config::Root;
use log4rs::encode::pattern::PatternEncoder;
use tempfile::tempdir;

/// Configures and initializes the logging system using log4rs.
fn main() -> Result<()> {
    let output_directory = tempdir()?;
    let output_path = output_directory.path().join("output.log");
    let logfile = file_appender(&output_path)?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))?;

    log4rs::init_config(config)?;

    log::info!("Hello, world!");

    println!("log_custom example: log4rs wrote to a temporary file");
    Ok(())
}

/// Builds a file appender for a caller-provided output path.
fn file_appender(path: impl AsRef<Path>) -> Result<FileAppender> {
    FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build(path)
        .map_err(Into::into)
}

// ANCHOR_END: example

pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the log custom example's main path succeeds.
    #[test]
    fn file_appender_creates_file_in_temporary_directory() -> anyhow::Result<()>
    {
        let directory = tempdir()?;
        let path = directory.path().join("output.log");
        let _appender = file_appender(&path)?;
        assert!(path.is_file());
        Ok(())
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
