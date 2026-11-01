//! Runs the immediate mode GUI example.
//!
//! TODO Reviewed ??-??-2026.

mod egui;
mod egui_headless;
mod egui_kittest;
mod egui_plot;
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
