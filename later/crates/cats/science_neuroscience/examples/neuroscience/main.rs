//! Runs the neuroscience example.
//!
//! TODO Reviewed ??-??-2026.

mod leaky_integrate_fire;
mod neuro;

/// Runs the neuroscience example.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    neuro::run()?;
    leaky_integrate_fire::run();
    Ok(())
}
