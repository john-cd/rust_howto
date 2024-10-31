// env_logger is simple logger that can be configured via environment
// variables. Example: RUST_LOG=info ./app

fn main() {
    // Typically you would use:
    // env_logger::init();

    // The `Env` lets us tweak what the environment
    // variables to read are and what the default
    // value is if they're missing
    // let env = env_logger::Env::default()
    //     .filter_or("MY_LOG_LEVEL", "trace") // Specify an environment
    // variable to read the filter from. If the variable is not set, the
    // default value will be used.     .write_style_or("MY_LOG_STYLE",
    // "always"); // Specify an environment variable to read the filter
    // from. If the variable is not set, the default value will be used.
    // env_logger::init_from_env(env);

    init_logger();

    log::info!("informational message");
    log::warn!("warning message");
    log::error!("this is an error {}", "message");
    if log::log_enabled!(log::Level::Info) {
        let x = 3 * 4; // "expensive" computation
        log::trace!("the answer was: {}", x);
    }
}

fn init_logger() {
    let _ = env_logger::builder()
            // Include all events in tests
            .filter_level(log::LevelFilter::max())
            // Ensure events are captured by `cargo test`
            .is_test(true)
            // Ignore errors initializing the logger if tests race to configure it
            .try_init();
}

#[test]
fn test() {
    main();
}
