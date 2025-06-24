#![allow(dead_code)]
// ANCHOR: example
mod foo {
    mod bar {
        /// Logs warning, info, and debug messages from the `bar` module.
        pub fn run() {
            log::warn!("[bar] warn");
            log::info!("[bar] info");
            log::debug!("[bar] debug");
        }
    }

    /// Logs warning, info, and debug messages from the `foo` module, and then
    /// calls `bar::run()`.
    pub fn run() {
        log::warn!("[foo] warn");
        log::info!("[foo] info");
        log::debug!("[foo] debug");
        bar::run();
    }
}

/// Initializes the logger and logs warning, info, and debug messages from the
/// root module, then calls `foo::run()`.
fn main() {
    env_logger::init();
    log::warn!("[root] warn");
    log::info!("[root] info");
    log::debug!("[root] debug");
    foo::run();
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
