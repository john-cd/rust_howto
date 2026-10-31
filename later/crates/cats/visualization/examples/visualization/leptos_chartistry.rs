// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates a Leptos component that renders a chart with
//! `leptos-chartistry`.
//!
//! Leptos is a web framework whose components re-render when their data changes
//! (a signal changes), compiled to WebAssembly to run in the browser.
//! Chartistry is a charting component for it.

#![expect(
    clippy::same_name_method,
    reason = "Leptos generates a builder method with the component's name."
)]

use leptos::prelude::*;
use leptos_chartistry::AspectRatio;
use leptos_chartistry::Chart;
use leptos_chartistry::Line;
use leptos_chartistry::Series;
use leptos_chartistry::TickLabels;
use leptos_chartistry::Tooltip;

/// One timestamp-free sample for the response-time chart.
#[derive(Clone, Copy, Debug)]
struct Measurement {
    request: f64,
    response_ms: f64,
}

/// Builds a responsive SVG chart for sample request response times.
#[component]
fn ResponseTimeChart() -> impl IntoView {
    let data: Signal<Vec<Measurement>> = Signal::derive(|| {
        vec![
            Measurement {
                request: 1.0,
                response_ms: 42.0,
            },
            Measurement {
                request: 2.0,
                response_ms: 55.0,
            },
            Measurement {
                request: 3.0,
                response_ms: 47.0,
            },
            Measurement {
                request: 4.0,
                response_ms: 36.0,
            },
        ]
    });

    view! {
        <Chart
            aspect_ratio=AspectRatio::from_outer_ratio(600.0, 300.0)
            bottom=TickLabels::aligned_floats()
            left=TickLabels::aligned_floats()
            tooltip=Tooltip::left_cursor()
            series=Series::new(|measurement: &Measurement| measurement.request)
                .line(Line::new(|measurement: &Measurement| measurement.response_ms)
                    .with_name("Response time"))
            data=data
        />
    }
}

/// Keeps the chart component available for mounting in a Leptos application.
fn main() {
    std::hint::black_box(ResponseTimeChart);
}
// ANCHOR_END: example

/// References the component from the visualization example entry point.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point.
    #[test]
    fn test_main_entry_point() {
        main();
        run();
    }

    /// The sample data points to a response time in milliseconds.
    #[test]
    fn measurement_holds_request_and_response_time() {
        let measurement = Measurement {
            request: 1.0,
            response_ms: 42.0,
        };
        assert!((measurement.response_ms - 42.0).abs() < f64::EPSILON);
        assert!((measurement.request - 1.0).abs() < f64::EPSILON);
    }
}
