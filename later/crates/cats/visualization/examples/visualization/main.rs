//! Runs the visualization example.
//!
//! TODO Reviewed ??-??-2026.

mod leptos_chartistry;
mod plotly;
mod plotters;

/// Runs the visualization example.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    plotly::run()?;
    plotters::run()?;
    leptos_chartistry::run();
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
