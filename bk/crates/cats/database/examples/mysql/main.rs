//! Runs the `mysql_async` MySQL example when the `mysql` feature is enabled.
//!
//! TODO Reviewed ??-??-2026.

#[cfg(feature = "mysql")]
mod mysql_async;

/// Runs the example if the `mysql` feature is enabled.
fn main() -> anyhow::Result<()> {
    #[cfg(feature = "mysql")]
    {
        mysql_async::run()?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Without the `mysql` feature, `main` has nothing to run and succeeds.
    #[cfg(not(feature = "mysql"))]
    #[test]
    fn test_main_is_a_no_op_without_the_feature() {
        assert!(main().is_ok());
    }

    /// Runs the example against a running MySQL server.
    #[cfg(feature = "mysql")]
    #[test]
    #[ignore = "requires a running MySQL server"]
    fn test_main_runs() {
        assert!(main().is_ok());
    }
}
