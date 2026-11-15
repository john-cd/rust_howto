//! Runs the visualization example.
//!
//! TODO Reviewed ??-??-2026.

/// Demonstrates a Leptos component that renders a chart with
mod leptos_chartistry;

/// This example demonstrates how to create a simple scatter plot using the
mod plotly;

/// Demonstrates rendering a 2D line plot into an SVG string using `plotters`.
mod plotters;

/// Runs the visualization example.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    plotly::run()?;
    plotters::run()?;
    leptos_chartistry::run();
    Ok(())
}
