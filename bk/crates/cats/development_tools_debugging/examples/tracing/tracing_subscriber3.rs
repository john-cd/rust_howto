#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to enable or
//! disable layers at runtime based on a configuration flag.
//!
//! It configures a `tracing` subscriber with multiple layers, including a
//! human-readable pretty-printed layer and an optional JSON-formatted layer
//! that logs to a file.

use std::fs::File;

use serde::Deserialize;
use tracing_subscriber::Registry;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;

/// Configuration struct.
#[derive(Debug, Deserialize)]
struct Config {
    is_prod: bool,
    path: String,
}

/// Demonstrates how to configure tracing at runtime.
fn main() -> anyhow::Result<()> {
    // Example configuration.
    // In production, this may be loaded from a configuration file or
    // environment variables.
    let cfg = Config {
        is_prod: true,
        path: "temp/log.json".into(),
    };

    // Setup a human-readable, multi-line logger.
    // `layer()` is a shorthand for the equivalent `Layer::default` function.
    let pretty_layer = fmt::layer().pretty();

    // If `cfg.is_prod` is true, also log JSON-formatted logs to a file.
    let json_optional_layer: Option<_> = if cfg.is_prod {
        let file = File::create(cfg.path)?;
        let json_layer = fmt::layer().json().with_writer(file);
        Some(json_layer)
    } else {
        None
    };

    let subscriber = Registry::default().with(pretty_layer);
    // A `Layer` wrapped in an `Option` also implements the `Layer` trait.
    // This allows individual layers to be enabled or disabled at runtime while
    // always producing a `Subscriber` of the same type.
    //
    // If `cfg.is_prod` is false, then `json_optional_layer` will be `None`, and
    // this layer will do nothing.
    let subscriber = subscriber.with(json_optional_layer);

    tracing::subscriber::set_global_default(subscriber)?;

    tracing::error!("tracing configured!");
    println!("Done.");

    Ok(())
}
// ANCHOR_END: example

use rusty_fork::rusty_fork_test;

// Runs in a separate process.
rusty_fork_test! {
    #[test]
    fn test() {
        use std::fs;
        if !fs::exists("temp")? {
            fs::create_dir("temp")?;
        }
        main().unwrap();
    }
}
