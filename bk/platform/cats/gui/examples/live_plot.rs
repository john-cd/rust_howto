// TODO Reviewed ??-??-2026.
//! Adds simulated samples to an interactive desktop plot.

// ANCHOR: example
use eframe::egui;
use egui_plot::Line;
use egui_plot::Plot;

/// Stores the samples plotted by the desktop app.
#[derive(Default)]
struct App {
    samples: Vec<[f64; 2]>,
}

impl App {
    /// Appends the next sample at an increasing x coordinate.
    fn sample(&mut self) {
        let tick = self.samples.len() as f64;
        self.samples.push([tick, (tick / 4.0).sin()]);
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if ui.button("Add sample").clicked() {
            self.sample();
        }
        Plot::new("samples").show(ui, |plot| {
            plot.line(Line::new("Signal", self.samples.clone()));
        });
    }
}

/// Opens a window for stepping through the simulated signal.
fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Signal samples",
        eframe::NativeOptions::default(),
        Box::new(|_| {
            let mut app = App::default();
            app.sample();
            Ok(Box::new(app))
        }),
    )
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    /// Verifies that adding a sample retains the earlier point.
    #[test]
    fn adds_ordered_samples() {
        let mut app = super::App::default();
        app.sample();
        app.sample();
        assert_eq!(app.samples[0], [0.0, 0.0]);
        assert_eq!(app.samples[1][0], 1.0);
        assert!(app.samples[1][1] > 0.0);
    }
}
