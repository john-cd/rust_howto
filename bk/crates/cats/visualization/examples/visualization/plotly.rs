// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates how to create a simple scatter plot using the
//! `plotly` crate.
//!
//! Plotly produces interactive charts (zoom, hover) as HTML with JavaScript.
//! The Rust crate builds the chart description, which the browser then renders.
//! A scatter plot shows the relationship between two variables as points.

use std::fs;
use std::path::Path;

use anyhow::Context;
use plotly::Layout;
use plotly::Plot;
use plotly::Scatter;
use plotly::common::Mode;
use plotly::layout::Axis;

/// Builds the HTML document for the sample scatter plot.
fn scatter_plot_html() -> String {
    let trace = Scatter::new(vec![1, 2, 3, 4, 5], vec![10, 15, 13, 17, 14])
        .mode(Mode::Markers)
        .name("Sample data");
    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.set_layout(
        Layout::new()
            .title("Sample readings")
            .x_axis(Axis::new().title("Sample number"))
            .y_axis(Axis::new().title("Reading")),
    );
    plot.to_html()
}

/// Saves the plot and reports the path when writing succeeds.
fn main() -> anyhow::Result<()> {
    let output = Path::new("temp/plot.html");
    fs::create_dir_all("temp").context("create plot output directory")?;
    fs::write(output, scatter_plot_html())
        .with_context(|| format!("write {}", output.display()))?;
    println!("Plot saved to {}", output.display());
    Ok(())
}
// ANCHOR_END: example

/// Creates a scatter plot and saves it to an HTML file.
///
/// # Errors
///
/// Returns an error if a step of the example fails.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::scatter_plot_html;

    /// Exports the samples, axis labels, and Plotly JavaScript bootstrap.
    #[test]
    fn exports_a_labeled_scatter_plot() {
        let html = scatter_plot_html();
        assert!(html.contains("Sample readings"));
        assert!(html.contains("Sample number"));
        assert!(html.contains("Reading"));
        assert!(html.contains("[10,15,13,17,14]"));
        assert!(html.contains("Plotly.newPlot"));
    }
}
