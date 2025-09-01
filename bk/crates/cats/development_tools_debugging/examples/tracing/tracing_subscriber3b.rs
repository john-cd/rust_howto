#![allow(dead_code)]
// ANCHOR: example
use std::fs::File;
use std::io;

use tracing_subscriber::Layer;
use tracing_subscriber::filter;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;
use tracing_subscriber::reload;
use tracing_subscriber::reload::Handle;

struct Config {
    enable_log_file: bool,
    enable_stdout: bool,
    enable_stderr: bool,
}

/// Demonstrates how to configure tracing at runtime,
/// - dealing with a variable number of `Layer`s of different types.
/// - reloading the tracing configuration at runtime.
fn main() -> anyhow::Result<()> {
    let cfg = Config {
        enable_log_file: true,
        enable_stdout: true,
        enable_stderr: true,
    };

    // We create below any number of layers, based on our dynamically loaded
    // config file. We box the layers, which are of varied types, as
    // type-erased trait objects, so that they could be pushed to the `Vec`.
    let mut layers: Vec<Box<dyn Layer<_> + Send + Sync>> = Vec::new();

    if cfg.enable_log_file {
        let file = File::create("temp/myapp.log")?;
        let layer = fmt::layer()
        .json()
        .with_writer(file)
        // More layer configuration here...
        .with_thread_names(true)
        .boxed(); // Equivalent to `Box::new`.
        layers.push(layer);
    }

    let mut reload_handle: Option<Handle<_, _>> = None;
    if cfg.enable_stdout {
        let layer = fmt::layer().pretty().with_filter(LevelFilter::WARN);
        // We wrap this `layer` using the `reload` module to get a reload
        // handle.
        let (layer, handle) = reload::Layer::new(layer);
        layers.push(Box::new(layer));
        reload_handle = Some(handle);
    }

    if cfg.enable_stderr {
        let layer = fmt::layer()
            .with_writer(io::stderr)
            .with_filter(LevelFilter::DEBUG)
            .boxed();
        layers.push(layer);
    }

    // `Vec<L> where L: Layer` implements `Layer`.
    // `Box<dyn Layer<S> + Send + Sync>` also implements `Layer`.
    tracing_subscriber::registry().with(layers).init();

    tracing::info!("This will be ignored by the pretty logger to stdout.");

    // Upon receiving a reload signal...
    if let Some(hdl) = reload_handle {
        hdl.modify(|layer| *layer.filter_mut() = filter::LevelFilter::INFO)?;
    }
    tracing::info!("This will be logged by all layers.");
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
