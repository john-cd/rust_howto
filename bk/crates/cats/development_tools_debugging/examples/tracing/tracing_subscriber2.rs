#![allow(dead_code)]
// ANCHOR: example
use std::io;

// use tracing_subscriber::filter;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
// use tracing_subscriber::prelude::*;
use tracing_subscriber::util::SubscriberInitExt;

/// The most important component of the `tracing-subscriber` API is the
/// `Layer` trait. `Layer`s can be composed together to form a
/// `Subscriber`.
fn layers() {
    // Create a few layers.
    // `layer()` is shorthand for the equivalent `Layer::default()` function.
    let layer1 = fmt::layer().with_writer(io::stderr); // Logs to the console's standard error stream.
    let layer2 = fmt::layer().with_writer(io::stdout); // Logs to the console.

    // Initialize tracing.
    // `registry()` is equivalent to `Registry::default()`.
    // A Registry is a Subscriber around which multiple Layers implementing
    // various behaviors may be added.
    tracing_subscriber::registry()
        .with(layer1)
        .with(layer2)
        .init();

    // Test it.
    tracing::debug!("tracing configured!");
}

use std::error::Error;

// A Layer which filters spans and events based on a set of filter directives.
// EnvFilter implements both the Layer and Filter traits, so it may be used for
// both global filtering and per-layer filtering, respectively.
#[allow(dead_code)]
fn filter() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // 1. Returns a new EnvFilter from the value of the RUST_LOG environment
    //    variable, ignoring any invalid filter directives.
    let _filter_layer = EnvFilter::from_default_env();

    // 2. Returns a new EnvFilter from the value of the RUST_LOG environment
    //    variable, or "info" if the environment variable is unset or contains
    //    any invalid filter directives.
    let _filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    // "development_tools_debugging=debug,axum=debug,tower_http=debug,
    // mongodb=debug" "my_crate=info,my_crate::my_mod=debug,
    // [my_span]=trace"

    // 3. Custom environment variable
    let _filter_layer = EnvFilter::try_from_env("MY_CUSTOM_FILTER_ENV_VAR")?
        // Set the base level when not matched by other directives to DEBUG.
        .add_directive(LevelFilter::DEBUG.into())
        // Set the max level for `my_crate::my_mod` to TRACE, overriding
        // any directives parsed from the env variable.
        .add_directive("my_crate::my_mod=trace".parse()?);

    // FIXME fmt().with_env_filter(filter_layer).try_init()?;

    // Test it.
    tracing::debug!("tracing configured!");
    Ok(())
}

fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    layers();
    // FIXME filter()?;
    Ok(())
}
// ANCHOR_END: example

use rusty_fork::rusty_fork_test;

// Runs in a separate process.
rusty_fork_test! {
    #[test]
    fn test() {
        main().unwrap();
    }
}
// FIXME NOW

// Per-Layer Filtering
// https://docs.rs/tracing-subscriber/latest/tracing_subscriber/layer/index.html#per-layer-filtering

// // Generates an HTTP access log.
// let access_log = // ...

// // Add a filter to the access log layer so that it only observes
// // spans and events with the `http_access` target.
// let access_log = access_log.with_filter(filter::filter_fn(|metadata| {
//     // Returns `true` if and only if the span or event's target is
//     // "http_access".
//     metadata.target() == "http_access"
// }));

// // A general-purpose logging layer.
// let fmt_layer = tracing_subscriber::fmt::layer();

// // Build a subscriber that combines the access log and stdout log
// // layers.
// tracing_subscriber::registry()
//     .with(fmt_layer)
//     .with(access_log)
//     .init();

// use tracing_subscriber::layer::SubscriberExt;
// use tracing_subscriber::util::SubscriberInitExt;

// fn main() {
//     tracing_subscriber::registry()
//         .with(tracing_subscriber::fmt::layer())
//         .with(tracing_subscriber::filter::EnvFilter::new(
//             std::env::var("RUST_LOG").unwrap_or_else(|_| {
//
// "myproj=debug,axum=debug,tower_http=debug,mongodb=debug".into()
// }),         ))
//         .init();
//     tracing::info!("tracing configured!");
//     println!("Done.")
// }
