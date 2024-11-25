// ANCHOR: example

fn main() {
    init_logger();

    log::info!("informational message");
    log::warn!("warning message");
    log::error!("this is an error {}", "message");
    if log::log_enabled!(log::Level::Info) {
        let x = 3 * 4; // "Expensive" computation
        log::trace!("the answer was: {}", x);
    }
}

//#[cfg(not(test))]
fn init_logger() {
    // env_logger is a simple logger that can be configured via environment
    // variables. Example: RUST_LOG=info ./app
    // Typically you would use:
    // env_logger::init();

    // Initialise a logger with filter level Off,
    // then override the log filter from an environment variable called
    // MY_APP_LOG:
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Off)
        .parse_env("MY_APP_LOG")
        .init();

    // Alternatively, `Env` lets us tweak what the environment
    // variables to read are and what the default
    // value is if they're missing
    // let env = env_logger::Env::default()
    // // Specify an environment variable to read the filter from.
    // // If the variable is not set, the default value will be used.
    // .filter_or("MY_APP_LOG", "trace")
    // .write_style_or("MY_APP_LOG_STYLE", "always");
    // env_logger::init_from_env(env);
}
// ANCHOR_END: example

// #[cfg(test)]
// fn init_logger() {
//     let _ = env_logger::builder()
//             // Include all events in tests
//             .filter_level(log::LevelFilter::max())
//             // Ensure events are captured by `cargo test`
//             .is_test(true)
//             // Ignore errors initializing the logger if tests race to
// configure it             .try_init();
// }

use rusty_fork::rusty_fork_test;
// Runs in a separate process
rusty_fork_test! {
    #[test]
    fn test() {
        main();
    }
}

// TODO test fully and review vs text
