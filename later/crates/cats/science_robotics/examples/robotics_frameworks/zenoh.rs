// TODO Reviewed ??-??-2026.

// Enabled by the `zenoh` Cargo feature. The advisory exception for the
// `lz4_flex` 0.10 series that Zenoh requires is documented in deny.toml.

// ANCHOR: example
//! Publishes and receives several local telemetry values with `zenoh`.
//!
//! Zenoh is a publish/subscribe protocol for robots and edge devices. Data is
//! published to hierarchical key expressions such as `robot/battery`, and
//! subscribers receive values for the keys they match, with no central broker
//! needed. It also supports queries, so one program can ask another for the
//! latest value.

use std::time::Duration;

use anyhow::Result;
use zenoh::Config;

const KEY_EXPR: &str = "science/robotics/telemetry";
const TELEMETRY: [&str; 3] =
    ["temperature=21.5", "temperature=21.7", "temperature=21.9"];

/// Opens a Zenoh session and prints the received telemetry values.
///
/// # Errors
///
/// Returns an error if the session cannot open or any telemetry message fails.
#[tokio::main]
async fn main() -> Result<()> {
    for payload in publish_and_receive(local_config()?).await? {
        println!("Received telemetry: {payload}");
    }

    Ok(())
}

/// Publishes telemetry and collects each matching sample from one session.
///
/// # Errors
///
/// Returns an error if Zenoh setup, publication, reception, or cleanup fails.
async fn publish_and_receive(config: Config) -> Result<Vec<String>> {
    let session = zenoh::open(config).await.map_err(zenoh_error)?;
    let subscriber = session
        .declare_subscriber(KEY_EXPR)
        .await
        .map_err(zenoh_error)?;
    let publisher = session
        .declare_publisher(KEY_EXPR)
        .await
        .map_err(zenoh_error)?;

    for payload in TELEMETRY {
        publisher.put(payload).await.map_err(zenoh_error)?;
    }

    let mut received = Vec::with_capacity(TELEMETRY.len());
    for _ in TELEMETRY {
        let sample = tokio::time::timeout(
            Duration::from_secs(5),
            subscriber.recv_async(),
        )
        .await
        .map_err(|_| anyhow::anyhow!("timed out waiting for telemetry"))?
        .map_err(zenoh_error)?;
        received.push(
            sample
                .payload()
                .try_to_string()
                .map_err(zenoh_error)?
                .into_owned(),
        );
    }
    received.sort();

    drop(publisher);
    subscriber.undeclare().await.map_err(zenoh_error)?;
    session.close().await.map_err(zenoh_error)?;
    Ok(received)
}

/// Returns a configuration that keeps the session on this machine: no
/// multicast scouting, and a listener on the IPv4 loopback address only.
///
/// # Errors
///
/// Returns an error if a configuration key is rejected.
fn local_config() -> Result<Config> {
    let mut config = Config::default();
    config
        .insert_json5("scouting/multicast/enabled", "false")
        .map_err(zenoh_error)?;
    config
        .insert_json5("listen/endpoints", r#"["tcp/127.0.0.1:0"]"#)
        .map_err(zenoh_error)?;
    Ok(config)
}

/// Converts a Zenoh error to the example's `anyhow` error type.
fn zenoh_error(error: impl std::fmt::Display) -> anyhow::Error {
    anyhow::anyhow!("{error}")
}
// ANCHOR_END: example

/// Runs the Zenoh pub/sub example.
///
/// # Errors
///
/// Returns an error if publishing or receiving telemetry fails.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the zenoh example through its entry point.
    #[test]
    fn zenoh_example_entry_point_runs() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }

    /// Verifies all telemetry messages are received by a local-only session.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn receives_all_local_telemetry_messages() -> Result<()> {
        let config = local_config()?;

        let mut expected = TELEMETRY.map(str::to_owned).to_vec();
        expected.sort();

        assert_eq!(publish_and_receive(config).await?, expected);
        Ok(())
    }
}
