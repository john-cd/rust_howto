//! Runs the `jetstream` NATS example when the `nats` feature is enabled.
//!
//! TODO Reviewed ??-??-2026.

/// Publishes orders to a NATS JetStream stream and processes them with a
#[cfg(feature = "nats")]
mod jetstream;

/// Runs the example if the `nats` feature is enabled.
fn main() -> anyhow::Result<()> {
    #[cfg(feature = "nats")]
    {
        jetstream::run()?;
    }
    Ok(())
}
