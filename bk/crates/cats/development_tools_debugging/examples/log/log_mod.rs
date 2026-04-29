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
    println!(
        "log_mod example: set RUST_LOG=debug to see module-specific log output above"
    );
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        main();
    }
}
