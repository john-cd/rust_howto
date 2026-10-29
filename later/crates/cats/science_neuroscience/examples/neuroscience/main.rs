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

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point (output-only: the modules
    /// test their own results).
    #[test]
    fn test_main_entry_point() {
        main().expect("the example should run successfully");
    }
}
