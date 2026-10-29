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
