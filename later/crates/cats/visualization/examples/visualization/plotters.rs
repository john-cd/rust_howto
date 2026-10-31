// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates rendering a 2D line plot into an SVG string using `plotters`.
//!
//! Plotters draws charts into many backends; SVG is a text format for vector
//! graphics that scales without blurring and can be embedded in web pages.
//! Drawing into a string avoids files.

use plotters::prelude::*;

/// Generates an SVG string containing a line chart plot.
fn generate_line_chart_svg()
-> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let mut buffer = String::new();

    {
        // Bind drawing area to the string buffer using the SVG backend.
        let root = SVGBackend::with_string(&mut buffer, (600, 400))
            .into_drawing_area();
        root.fill(&WHITE)?;

        // Build Cartesian chart context with coordinate ranges and label space.
        let mut chart = ChartBuilder::on(&root)
            .caption("Y = X^2", ("sans-serif", 30).into_font())
            .margin(10)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(-5.0f32..5.0f32, 0.0f32..25.0f32)?;

        chart.configure_mesh().draw()?;

        // Draw line series for Y = X^2.
        chart.draw_series(LineSeries::new(
            (-50..=50).map(|x| x as f32 / 10.0).map(|x| (x, x * x)),
            &RED,
        ))?;

        root.present()?;
    }

    Ok(buffer)
}

/// Renders the chart without opening a window or writing an output file.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let svg_content = generate_line_chart_svg()?;
    println!("Generated SVG length: {} bytes.", svg_content.len());
    Ok(())
}
// ANCHOR_END: example

/// Runs the in-memory SVG rendering example.
pub fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that the line chart is rendered into an SVG string containing
    /// expected elements.
    #[test]
    fn test_generate_line_chart_svg() {
        let svg =
            generate_line_chart_svg().expect("SVG generation should succeed.");
        assert!(
            svg.contains("<svg"),
            "Generated output should be valid SVG containing '<svg'."
        );
        assert!(
            svg.contains("Y = X^2"),
            "Generated SVG should contain caption title text."
        );
        assert!(
            svg.contains("<polyline"),
            "the SVG must contain the plotted series"
        );
    }

    /// Entry point test executing main().
    #[test]
    fn test_main() {
        main().expect("main should execute successfully.");
    }
}
