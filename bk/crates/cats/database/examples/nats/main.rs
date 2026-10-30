//! Runs the `jetstream` NATS example when the `nats` feature is enabled.
//!
//! TODO Reviewed ??-??-2026.

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

#[cfg(test)]
mod tests {
    use super::*;

    /// The entry point runs; with the `nats` feature it needs a local server
    /// only when `NATS_URL` is set, and otherwise exits quietly.
    #[test]
    fn test_main_entry_point() {
        main().expect("main should succeed without a broker configured");
    }
}
