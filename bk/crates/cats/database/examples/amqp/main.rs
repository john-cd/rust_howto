//! Runs the `lapin` AMQP example when the `lapin` feature is enabled.
//!
//! TODO Reviewed ??-??-2026.

#[cfg(feature = "lapin")]
mod lapin;

/// Runs the example if the `lapin` feature is enabled.
fn main() -> anyhow::Result<()> {
    #[cfg(feature = "lapin")]
    {
        lapin::run()?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Without the `lapin` feature, `main` has nothing to run and succeeds.
    #[cfg(not(feature = "lapin"))]
    #[test]
    fn test_main_is_a_no_op_without_the_feature() {
        assert!(main().is_ok());
    }

    /// Runs the example against a running RabbitMQ server.
    #[cfg(feature = "lapin")]
    #[test]
    #[ignore = "requires a running RabbitMQ server"]
    fn test_main_runs() {
        assert!(main().is_ok());
    }
}
