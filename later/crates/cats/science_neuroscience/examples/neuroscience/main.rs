//! Runs the neuroscience example.
//!
//! TODO Reviewed ??-??-2026.

/// Example: simulate a leaky integrate-and-fire neuron with constant current.
mod leaky_integrate_fire;

/// Example: create a dummy neuroimaging volume and save it as a `NIfTI` file.
mod neuro;

/// Runs the neuroscience example.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    neuro::run()?;
    leaky_integrate_fire::run();
    Ok(())
}
