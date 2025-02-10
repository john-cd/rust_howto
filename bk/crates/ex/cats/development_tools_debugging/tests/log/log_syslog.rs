// ANCHOR: example

#[cfg(target_os = "linux")]
fn main() -> anyhow::Result<()> {
    use syslog::Facility;

    syslog::init(
        Facility::LOG_USER,
        log::LevelFilter::Debug,
        Some("My app name"),
    )?;
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
