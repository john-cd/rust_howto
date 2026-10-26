#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Runs an `egui` UI for one frame without a window, a GPU or a display.
//!
//! The same technique lets the examples of the other `egui` add-on crates run
//! in continuous integration.

use eframe::egui;

/// Runs `add_contents` for one frame on a virtual 800 x 600 screen.
///
/// Returns the output of the frame: the shapes to paint, the platform
/// commands such as the clipboard text, and so on.
pub fn run_frame(
    ctx: &egui::Context,
    add_contents: impl FnMut(&mut egui::Ui),
) -> egui::FullOutput {
    let screen =
        egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(800.0, 600.0));
    let input = egui::RawInput {
        screen_rect: Some(screen),
        ..Default::default()
    };
    ctx.run_ui(input, add_contents)
}

/// Draws a label and a button, then looks at what was painted.
fn main() {
    let ctx = egui::Context::default();
    let mut clicked = false;
    let output = run_frame(&ctx, |ui| {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.label("Hello");
            // Without input events, the button is never clicked.
            clicked = ui.button("Click me").clicked();
        });
    });

    assert!(!clicked);
    // The frame produced something to paint.
    assert!(!output.shapes.is_empty());
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
