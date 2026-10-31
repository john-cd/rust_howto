// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Composing a `tracing` subscriber from layers and filters.
//!
//! Layers add behaviour to a subscriber and stack: one layer might print to the
//! terminal, another write JSON to a file, and a filter drop noisy modules.
//! Each layer sees every event.

use std::error::Error;
use std::io;

use tracing_subscriber::Layer as _;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;

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
    // A `Registry` is a `Subscriber`, around which multiple Layers implementing
    // various behaviors may be added.
    tracing_subscriber::registry()
        .with(layer1)
        .with(layer2)
        .init();

    // Test it.
    tracing::debug!("tracing configured!");
}

// A `Layer`, which filters spans and events based on a set of filter
// directives. `EnvFilter` implements both the `Layer` and `Filter` traits, so
// it may be used for both global filtering and per-layer filtering,
// respectively.
/// Shows several ways to build an `EnvFilter`.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "alternative global-subscriber setup; only one subscriber can be installed per process"
    )
)]
fn filter() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // 1. Returns a new `EnvFilter` from the value of the `RUST_LOG` environment
    //    variable, ignoring any invalid filter directives.
    let _filter_layer1 = EnvFilter::from_default_env();

    // 2. Returns a new `EnvFilter` from the value of the `RUST_LOG` environment
    //    variable, or "info" if the environment variable is unset or contains
    //    any invalid filter directives.
    let _filter_layer2 = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .expect("the tracing subscriber layers example invariant should hold");

    // "development_tools_debugging=debug,axum=debug,tower_http=debug,
    // mongodb=debug" "my_crate=info,my_crate::my_mod=debug,
    // [my_span]=trace"

    // 3. Custom environment variable.
    let filter_layer = EnvFilter::try_from_env("MY_CUSTOM_FILTER_ENV_VAR")?
        // Set the base level when not matched by other directives to `DEBUG`.
        .add_directive(LevelFilter::DEBUG.into())
        // Set the max level for `my_crate::my_mod` to `TRACE`, overriding
        // any directives parsed from the env variable.
        .add_directive("my_crate::my_mod=trace".parse()?);

    fmt().with_env_filter(filter_layer).try_init()?;

    // Test it.
    tracing::debug!("tracing configured!");
    Ok(())
}

/// ## Per-Layer Filtering
///
/// This example demonstrates how to add a filter to a specific layer.
/// This allows different layers to have different filtering rules.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "alternative global-subscriber setup; only one subscriber can be installed per process"
    )
)]
fn per_layer_filtering() {
    use tracing_subscriber::filter;

    // A general-purpose logging layer.
    let fmt_layer = tracing_subscriber::fmt::layer();

    // Add a filter to the access log layer so that it only observes
    // spans and events with the `http_access` target.
    let access_log = tracing_subscriber::fmt::layer()
        .with_target(false)
        .with_filter(filter::filter_fn(|metadata| {
            // Returns `true` if and only if the span or event's target is
            // "http_access".
            metadata.target() == "http_access"
        }));

    // Build a subscriber that combines the access log and stdout log
    // layers.
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(access_log)
        .init();

    tracing::info!(target: "http_access", "This is an access log message");
    tracing::info!("This is a general log message");
}

/// Initializes the layered subscriber.
fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // These are alternative global-subscriber setups; use one at a time.
    layers();
    Ok(())
}
// ANCHOR_END: example

pub fn run() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    main()
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    use super::*;
    use crate::isolated;

    /// Exercises the tracing layer configuration.
    #[test]
    fn test_layers() {
        isolated::run(module_path!(), "test_layers", || {
            layers();
        });
    }

    /// Checks that filters can be built.
    #[test]
    fn test_filter() -> std::io::Result<()> {
        let output = Command::new(std::env::current_exe()?)
            .args(["child_checks_custom_filter", "--ignored"])
            .env("MY_CUSTOM_FILTER_ENV_VAR", "info")
            .output()?;
        assert!(
            output.status.success(),
            "child filter test failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        Ok(())
    }

    /// Builds the filter in a fresh process with the environment from its
    /// parent.
    #[test]
    #[ignore = "needs a parent-controlled filter environment"]
    fn child_checks_custom_filter() {
        filter().expect(
            "the tracing subscriber layers example invariant should hold",
        );
    }

    /// Verifies that per layer filtering.
    #[test]
    fn test_per_layer_filtering() {
        isolated::run(module_path!(), "test_per_layer_filtering", || {
            per_layer_filtering();
        });
    }

    #[test]
    fn test_main_entry_point() {
        isolated::run(module_path!(), "test_main_entry_point", || {
            if let Err(err) = main() {
                panic!("main failed: {err:?}");
            }
        });
    }
}
