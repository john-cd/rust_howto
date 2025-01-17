// ANCHOR: example
use plotly::Plot;
use plotly::Scatter;
use plotly::common::Mode;

fn main() {
    // Create sample data
    let x_data = vec![1, 2, 3, 4, 5];
    let y_data = vec![10, 15, 13, 17, 14];

    // Create a Scatter plot
    let trace = Scatter::new(x_data, y_data)
        .mode(Mode::LinesMarkers)
        .name("Sample Data");

    // Create a Plot and add the Scatter plot
    let mut plot = Plot::new();
    plot.add_trace(trace);

    // Save the plot as an HTML file
    plot.write_html("temp/plot.html");

    println!("Plot saved as plot.html");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// [P1](https://github.com/john-cd/rust_howto/issues/884
