//! Runs the immediate mode GUI example.
//!
//! TODO Reviewed ??-??-2026.

mod egui;
mod egui_headless;
mod egui_kittest;
mod egui_log_viewer;
mod egui_plot;
mod egui_table;
mod egui_tiles;
mod egui_toasts_icons;

/// Runs the immediate mode gui example.
fn main() -> Result<(), eframe::Error> {
    egui::run()?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
