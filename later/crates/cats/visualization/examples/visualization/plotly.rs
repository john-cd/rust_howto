#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to create a simple scatter plot using the
//! `plotly` crate.

use std::fs;

use plotly::Layout;
use plotly::Plot;
use plotly::Scatter;
use plotly::common::Mode;

fn main() -> anyhow::Result<()> {
    // Sample data.
    let x_data = vec![1, 2, 3, 4, 5];
    let y_data = vec![10, 15, 13, 17, 14];

    // Create a `Scatter` plot.
    let trace = Scatter::new(x_data, y_data)
        .mode(Mode::LinesMarkers)
        .name("Sample Data");

    // Create a `Plot` and add the `Scatter` plot.
    // `Plot` is a container for structs that implement the `Trace` trait.
    // Optionally a `Layout` can also be specified. Its function is to serialize
    // `Trace`s and the `Layout` in html format and display and/or persist the
    // resulting plot.
    let mut plot = Plot::new();
    plot.add_trace(trace);

    let layout = Layout::new().title("<b>Scatter Plot</b>");
    plot.set_layout(layout);

    if !fs::exists("temp")? {
        fs::create_dir("temp")?;
    }
    // Save the plot as an HTML file.
    plot.write_html("temp/plot.html");

    println!("Plot saved as plot.html");
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
// [review how to test](https://github.com/john-cd/rust_howto/issues/884
