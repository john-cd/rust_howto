// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Drives a counter with pointer and keyboard events through `egui_kittest`.
//!
//! `egui_kittest` runs the UI headless, like the previous recipe, and adds
//! queries based on the accessibility tree (AccessKit), so a test reads like
//! a user: "click the button labelled Increment".

use eframe::egui;
use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;

/// Draws a counter with an `Increment` button.
///
/// # Arguments
///
/// * `ui` - egui interface used to draw the label and button.
/// * `count` - Counter incremented when the button is clicked.
fn counter_ui(ui: &mut egui::Ui, count: &mut u32) {
    ui.label(format!("Count: {count}"));
    if ui.button("Increment").clicked() {
        *count += 1;
    }
}

/// Checks pointer clicks, keyboard activation and state after resizing.
fn main() {
    // The state lives in the harness. The closure draws one frame at a time.
    let mut harness = Harness::new_ui_state(counter_ui, 0_u32);

    // Widgets are found by their accessible label.
    assert!(harness.query_by_label("Count: 0").is_some());

    harness.get_by_label("Increment").click();
    // `run` processes the pending events and repaints until the UI is stable.
    harness.run();
    harness.get_by_label("Increment").click();
    harness.run();

    assert!(harness.query_by_label("Count: 2").is_some());
    assert_eq!(*harness.state(), 2);

    // Use a fresh UI to verify keyboard-only interaction.
    let mut keyboard = Harness::new_ui_state(counter_ui, 0_u32);
    keyboard.key_press(egui::Key::Tab);
    keyboard.run();
    keyboard.key_press(egui::Key::Enter);
    keyboard.run();
    assert_eq!(*keyboard.state(), 1);

    // A narrower viewport preserves the user's state and the button label.
    keyboard.set_size(egui::vec2(240.0, 160.0));
    keyboard.run();
    assert!(keyboard.query_by_label("Count: 1").is_some());
    assert!(keyboard.query_by_label("Increment").is_some());
}
// ANCHOR_END: example

/// Runs the counter interaction checks.
pub fn run() {
    // Call through a binding: calling `main` directly trips
    // `clippy::main_recursion`.
    let entry = main;
    entry();
}

/// Tests behavior implemented by the surrounding module.
#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the egui kittest example through its entry point.
    #[test]
    fn test_egui_kittest_example_entry_point() {
        main();
    }
}
