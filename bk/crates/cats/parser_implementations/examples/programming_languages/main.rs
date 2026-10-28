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
