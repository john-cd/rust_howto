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

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
