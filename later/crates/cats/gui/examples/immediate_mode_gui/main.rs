//! Runs the immediate mode GUI example.
//!
//! TODO Reviewed ??-??-2026.

/// Simple example of a GUI application with `egui`.
mod egui;
/// Runs an `egui` UI for one frame without a window, a GPU or a display.
mod egui_headless;
/// Finds a button by its label and clicks it with `egui_kittest`.
mod egui_kittest;
/// A line plot and a bar chart with `egui_plot`.
mod egui_plot;
/// A virtualized data table with `egui_extras`.
mod egui_table;

/// Runs the immediate mode gui example.
fn main() -> Result<(), eframe::Error> {
    // Exercise the headless examples before opening the desktop window.
    egui_headless::run();
    egui_kittest::run();
    egui_plot::run();
    egui_table::run();
    egui::run()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the dispatcher through its entry point.
    #[test]
    #[ignore = "needs a desktop session that can open an egui window"]
    fn test_main_entry_point() {
        assert!(main().is_ok());
    }
}
