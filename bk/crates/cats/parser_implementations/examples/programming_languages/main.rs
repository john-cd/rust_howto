//! Runs the SQL parser example when the `sqlparser` feature is enabled.
//!
//! TODO Reviewed ??-??-2026.

#[cfg(feature = "sqlparser")]
mod sqlparser;
mod syn;

/// Runs the example if the `sqlparser` feature is enabled.
fn main() -> anyhow::Result<()> {
    syn::run()?;
    #[cfg(feature = "sqlparser")]
    {
        sqlparser::run()?;
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
