// ANCHOR: example
use std::io::Write;

use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;

fn main() {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    log::warn!("warn");
    log::info!("info");
    log::debug!("debug");
}

// ANCHOR_END: example
use rusty_fork::rusty_fork_test;
// Runs in a separate process
rusty_fork_test! {
#[test]
fn test() {
    main();
}
}
