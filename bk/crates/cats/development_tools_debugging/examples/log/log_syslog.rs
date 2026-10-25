#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Initializes syslog and logs debug and error messages.

#![cfg(unix)]

use syslog::Facility;

/// Initializes syslog and logs debug and error messages.
fn main() -> anyhow::Result<()> {
    // Initializes syslog with the user facility, debug level filter, and
    // application name.
    syslog::init(
        Facility::LOG_USER,
        log::LevelFilter::Debug,
        Some("My app name"),
    )?;

    // Logs a debug and error message.
    log::debug!("this is a debug {}", "message");
    log::error!("this is an error!");
    Ok(())
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "requires a Unix syslog daemon"]
    fn test_main() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
