// ANCHOR: example
//! # Zero Overhead Pub/Sub/Query Protocol with `zenoh`.
//!
//! This example demonstrates basic publish-subscribe messaging using Zenoh.
//! Zenoh provides a unified protocol for data in motion, data in-use, and
//! data at rest. Unlike MQTT and DDS, Zenoh is designed for both cloud and
//! edge computing, offering zero-copy and schema-less communication with
//! near-zero overhead.

use std::time::Duration;

use zenoh::Config;

/// Demonstrates basic Zenoh pub/sub in a single session.
///
/// A Zenoh session is the entry point for any Zenoh network operation.
/// Publishers and subscribers are declared on a session and communicate
/// via key expressions (hierarchical paths similar to MQTT topics).
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let key_expr = "science/robotics/status";

    // Open a Zenoh session. The default config uses peer-to-peer mode,
    // which supports local (same-session) routing without a broker.
    let session = zenoh::open(Config::default()).await.map_err(ze)?;
    println!("Session opened.");

    // Declare a subscriber for the key expression.
    // The subscriber must be declared before publishing to receive messages.
    let subscriber = session.declare_subscriber(key_expr).await.map_err(ze)?;
    println!("Subscriber declared on '{key_expr}'.");

    // Publish a message to the key expression.
    // Zenoh dispatches the message to all matching local and remote
    // subscribers.
    session.put(key_expr, "Hello, Zenoh!").await.map_err(ze)?;
    println!("Published to '{key_expr}'.");

    // Receive the next message with a timeout.
    let timeout = Duration::from_secs(5);
    match tokio::time::timeout(timeout, subscriber.recv_async()).await {
        Ok(Ok(sample)) => {
            let payload = sample
                .payload()
                .try_to_string()
                .unwrap_or_else(|e| e.to_string().into());
            println!(
                "Received '{}' from '{}'",
                payload,
                sample.key_expr().as_str()
            );
        }
        Ok(Err(e)) => {
            return Err(anyhow::anyhow!("Subscriber error: {e}"));
        }
        Err(_) => {
            return Err(anyhow::anyhow!("Timed out waiting for message"));
        }
    }

    // Undeclaring the subscriber and closing the session releases resources.
    subscriber.undeclare().await.map_err(ze)?;
    session.close().await.map_err(ze)?;
    Ok(())
}

/// Convert a zenoh error into an `anyhow::Error`.
///
/// Zenoh uses `Box<dyn std::error::Error + Send + Sync>` which lacks the
/// `'static` bound that `anyhow` requires for automatic `From` conversions.
fn ze(e: impl std::fmt::Display) -> anyhow::Error {
    anyhow::anyhow!("{e}")
}
// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_main() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
// [write](https://github.com/john-cd/rust_howto/issues/845)
// <https://github.com/eclipse-zenoh/zenoh/tree/main/examples>
// <https://drive.google.com/file/d/1ETSLz2ouJ2o9OpVvEoXrbGcCvpF4TwJy/view>
// compare to MQTT (Message Queueing Telemetry Transport) and DDS (Data
// Distribution Service)
