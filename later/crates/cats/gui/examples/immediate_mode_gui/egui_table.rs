// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A virtualized data table with `egui_extras`.
//!
//! A virtualized table only draws the rows currently visible, so it stays fast
//! with millions of rows. `egui` is an immediate-mode GUI: the interface is
//! rebuilt by code on every frame.

use eframe::egui;
use egui_extras::Column;
use egui_extras::TableBuilder;

/// Draws a table of `row_count` rows and returns how many rows were built.
fn show_table(ui: &mut egui::Ui, row_count: usize) -> usize {
    let mut built = 0;
    TableBuilder::new(ui)
        // Alternate the background of the rows.
        .striped(true)
        // The user can drag the border of a column header.
        .resizable(true)
        .column(Column::auto())
        .column(Column::initial(120.0).at_least(40.0))
        .column(Column::remainder())
        .header(20.0, |mut header| {
            header.col(|ui| {
                ui.strong("Row");
            });
            header.col(|ui| {
                ui.strong("Square");
            });
            header.col(|ui| {
                ui.strong("Parity");
            });
        })
        .body(|body| {
            // `rows` calls the closure only for the rows that are visible,
            // so 10 000 rows cost the same as a screenful.
            // The row height must be the same for all rows.
            body.rows(18.0, row_count, |mut row| {
                built += 1;
                let index = row.index();
                row.col(|ui| {
                    ui.label(index.to_string());
                });
                row.col(|ui| {
                    ui.label((index * index).to_string());
                });
                row.col(|ui| {
                    ui.label(if index % 2 == 0 { "even" } else { "odd" });
                });
            });
        });
    built
}

/// The application, to run in a window.
struct TableApp;

impl eframe::App for TableApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            show_table(ui, 10_000);
        });
    }
}

/// Opens a window that shows the table. It needs a display.
#[expect(dead_code, reason = "requires a GUI window and display")]
fn run_in_window() -> eframe::Result {
    eframe::run_native(
        "Table",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(TableApp))),
    )
}

/// Lays out the table for one frame without a window.
fn main() {
    let ctx = egui::Context::default();
    let mut built = 0;
    let screen =
        egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(800.0, 600.0));
    let input = egui::RawInput {
        screen_rect: Some(screen),
        ..Default::default()
    };
    let mut output = ctx.run_ui(input, |ui| {
        egui::CentralPanel::default().show(ui, |ui| {
            built = show_table(ui, 10_000);
        });
    });
    output.textures_delta.clear();

    // Only the visible rows were built.
    assert!(built > 0);
    assert!(built < 100);
    println!("{built} of 10000 rows were built");
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the egui table example through its entry point.
    #[test]
    fn test_egui_table_example_entry_point() {
        main();
    }

    #[test]
    fn short_table_builds_every_row() {
        let ctx = egui::Context::default();
        let mut built = 0;
        let _output = crate::egui_headless::run_frame(&ctx, |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                built = show_table(ui, 3);
            });
        });
        assert_eq!(built, 3);
    }
}
