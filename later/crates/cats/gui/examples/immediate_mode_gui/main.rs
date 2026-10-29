//! Runs the immediate mode GUI example.
//!
//! TODO Reviewed ??-??-2026.

mod egui;
mod egui_headless;
mod egui_kittest;
#[cfg(feature = "egui_legacy")]
mod egui_log_viewer;
mod egui_plot;
mod egui_table;
#[cfg(feature = "egui_legacy")]
mod egui_tiles;
#[cfg(feature = "egui_legacy")]
mod egui_toasts_icons;

/// Runs the immediate mode gui example.
fn main() -> Result<(), eframe::Error> {
    egui::run()?;
    Ok(())
}
