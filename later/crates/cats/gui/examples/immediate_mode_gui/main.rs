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
    egui::run()?;
    Ok(())
}
