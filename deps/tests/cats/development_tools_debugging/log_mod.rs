// ANCHOR: example
mod foo {
    mod bar {
        pub fn run() {
            log::warn!("[bar] warn");
            log::info!("[bar] info");
            log::debug!("[bar] debug");
        }
    }

    pub fn run() {
        log::warn!("[foo] warn");
        log::info!("[foo] info");
        log::debug!("[foo] debug");
        bar::run();
    }
}

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
