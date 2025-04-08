// ANCHOR: example
use syslog::Facility;

/// Initializes syslog and logs debug and error messages.
#[cfg(target_os = "linux")]
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

#[cfg(not(target_os = "linux"))]
fn main() -> anyhow::Result<()> {
    println!("So far, only Linux systems are supported.");
    Ok(())
}
// ANCHOR_END: example

use rusty_fork::rusty_fork_test;

// Runs in a separate process
rusty_fork_test! {
    #[test]
    fn test() {
        main().unwrap();
    }
}
