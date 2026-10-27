// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Publishes and receives several local telemetry values with `zenoh`.

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
    for payload in publish_and_receive(Config::default()).await? {
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
    let mut subscriber = session
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

    /// Verifies all telemetry messages are received with multicast scouting
    /// off.
    #[tokio::test]
    async fn receives_all_local_telemetry_messages() -> Result<()> {
        let mut config = Config::default();
        config
            .insert_json5("scouting/multicast/enabled", "false")
            .map_err(zenoh_error)?;

        let mut expected = TELEMETRY.map(str::to_owned).to_vec();
        expected.sort();

        assert_eq!(publish_and_receive(config).await?, expected);
    }
}
