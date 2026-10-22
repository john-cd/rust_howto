//! Runs the `lapin` AMQP example when the `lapin` feature is enabled.
//!
//! TODO Reviewed ??-??-2026

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
