//! Runs the immediate mode GUI example.
//!
//! TODO Reviewed ??-??-2026

mod egui;

/// Runs the example from the `egui` module.
fn main() -> Result<(), eframe::Error> {
    egui::run()?;
    Ok(())
}
