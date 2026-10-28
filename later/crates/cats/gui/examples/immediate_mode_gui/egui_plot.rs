// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A line plot and a bar chart with `egui_plot`.

use eframe::egui;
use egui_plot::Bar;
use egui_plot::BarChart;
use egui_plot::Line;
use egui_plot::Plot;
use egui_plot::PlotPoints;
use egui_plot::VLine;

/// Samples a sine wave between 0 and 2 pi.
fn sine_points() -> Vec<[f64; 2]> {
    (0..=100)
        .map(|i| {
            let x = f64::from(i) * std::f64::consts::TAU / 100.0;
            [x, x.sin()]
        })
        .collect()
}

/// Draws a line plot, with a vertical marker at pi.
fn show_line_plot(ui: &mut egui::Ui) -> egui_plot::PlotResponse<()> {
    Plot::new("sine").height(200.0).show(ui, |plot_ui| {
        // `Vec<[f64; 2]>` converts into `PlotPoints`.
        let points = PlotPoints::from(sine_points());
        plot_ui.line(Line::new("sin(x)", points));
        plot_ui.vline(VLine::new("pi", std::f64::consts::PI));
    })
}

/// Draws a bar chart.
fn show_bar_chart(ui: &mut egui::Ui) -> egui_plot::PlotResponse<()> {
    let bars = vec![
        Bar::new(1.0, 3.0).name("Mon"),
        Bar::new(2.0, 5.0).name("Tue"),
        Bar::new(3.0, 2.0).name("Wed"),
    ];
    Plot::new("bars").height(200.0).show(ui, |plot_ui| {
        plot_ui.bar_chart(BarChart::new("visits", bars).width(0.6));
    })
}

/// Lays out both plots for one frame without a window.
fn main() {
    let ctx = egui::Context::default();
    let screen =
        egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(800.0, 600.0));
    let input = egui::RawInput {
        screen_rect: Some(screen),
        ..Default::default()
    };
    let mut line_bounds = None;
    let mut bar_bounds = None;
    let _output = ctx.run_ui(input, |ui| {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            line_bounds = Some(*show_line_plot(ui).transform.bounds());
            bar_bounds = Some(*show_bar_chart(ui).transform.bounds());
        });
    });

    // The plots fit their bounds to the data.
    let line = line_bounds.expect("the line plot was shown");
    assert!(line.min()[0] <= 0.0 && line.max()[0] >= std::f64::consts::TAU);
    assert!(line.min()[1] <= -1.0 && line.max()[1] >= 1.0);
    let bars = bar_bounds.expect("the bar chart was shown");
    assert!(bars.min()[0] <= 1.0 && bars.max()[0] >= 3.0);
    assert!(bars.max()[1] >= 5.0);
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the egui plot example through its entry point.
    #[test]
    fn test_egui_plot_example_entry_point() {
        main();
    }

    #[test]
    fn sine_has_101_points() {
        let points = sine_points();
        assert_eq!(points.len(), 101);
        assert!(points[0][1].abs() < 1e-12);
    }
}
