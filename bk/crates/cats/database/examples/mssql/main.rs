//! Runs the `tiberius` SQL Server example when the `mssql` feature is enabled.
//!
//! TODO Reviewed ??-??-2026.

#[cfg(feature = "mssql")]
mod tiberius;

/// Runs the example if the `mssql` feature is enabled.
fn main() -> anyhow::Result<()> {
    #[cfg(feature = "mssql")]
    {
        tiberius::run()?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Without the `mssql` feature, `main` has nothing to run and succeeds.
    #[cfg(not(feature = "mssql"))]
    #[test]
    fn test_main_is_a_no_op_without_the_feature() {
        assert!(main().is_ok());
    }

    /// Runs the example against a running SQL Server instance.
    #[cfg(feature = "mssql")]
    #[test]
    #[ignore = "requires a running SQL Server instance"]
    fn test_main_runs() {
        assert!(main().is_ok());
    }
}
