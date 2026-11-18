// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Mounts a reactive Chartistry line chart in the browser.

#[cfg(target_arch = "wasm32")]
mod browser {
    use leptos::prelude::*;
    use leptos_chartistry::AspectRatio;
    use leptos_chartistry::Chart;
    use leptos_chartistry::Line;
    use leptos_chartistry::RotatedLabel;
    use leptos_chartistry::Series;
    use leptos_chartistry::TickLabels;
    use leptos_chartistry::Tooltip;

    /// A request number and its response time in milliseconds.
    #[derive(Clone, Copy)]
    struct Measurement {
        request: f64,
        response_ms: f64,
    }

    /// Mounts the chart and its sample control into the document body.
    pub fn mount() {
        leptos::mount::mount_to_body(response_time_chart);
    }

    /// Draws the current samples and updates the line when the reader adds one.
    fn response_time_chart() -> impl IntoView {
        let show_extra = RwSignal::new(false);
        let data = Signal::derive(move || {
            let mut readings = vec![
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
            ];
            if show_extra.get() {
                readings.push(Measurement {
                    request: 5.0,
                    response_ms: 49.0,
                });
            }
            readings
        });

        view! {
            <main>
                <h1>"Response times"</h1>
                <button
                    type="button"
                    on:click=move |_| show_extra.update(|visible| *visible = !*visible)
                >
                    {move || if show_extra.get() { "Remove sample" } else { "Add sample" }}
                </button>
                <p role="status">{move || format!("{} samples", data.get().len())}</p>
                <Chart
                    aspect_ratio=AspectRatio::from_outer_ratio(640.0, 320.0)
                    bottom=RotatedLabel::middle("Request number")
                    left=RotatedLabel::middle("Response time (ms)")
                    top=TickLabels::aligned_floats()
                    right=TickLabels::aligned_floats()
                    tooltip=Tooltip::left_cursor()
                    series=Series::new(|sample: &Measurement| sample.request)
                        .line(Line::new(|sample: &Measurement| sample.response_ms)
                            .with_name("Response time"))
                    data=data
                />
            </main>
        }
    }
}

/// Starts the browser application or prints launch instructions on a native target.
fn main() {
    #[cfg(target_arch = "wasm32")]
    browser::mount();

    #[cfg(not(target_arch = "wasm32"))]
    println!(
        "From bk/crates/cats/visualization, run: trunk serve examples/chartistry/index.html --locked"
    );
}
// ANCHOR_END: example
