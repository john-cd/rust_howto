//! Runs the Elasticsearch example when the `elasticsearch` feature is enabled.
//!
//! TODO Reviewed ??-??-2026.

#[cfg(feature = "elasticsearch")]
mod elasticsearch;

/// Runs the example if the `elasticsearch` feature is enabled.
fn main() -> anyhow::Result<()> {
    #[cfg(feature = "elasticsearch")]
    {
        elasticsearch::run()?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Without the `elasticsearch` feature, `main` has nothing to run and
    /// succeeds.
    #[cfg(not(feature = "elasticsearch"))]
    #[test]
    fn test_main_is_a_no_op_without_the_feature() {
        assert!(main().is_ok());
    }

    /// Runs the example against a running Elasticsearch server.
    #[cfg(feature = "elasticsearch")]
    #[test]
    #[ignore = "requires a running Elasticsearch server"]
    fn test_main_runs() {
        assert!(main().is_ok());
    }
}
