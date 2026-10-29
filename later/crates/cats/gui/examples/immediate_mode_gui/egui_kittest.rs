// TODO Reviewed ??-??-2026.

#![allow(
    dead_code,
    reason = "the snippet declares items to illustrate them, not to call them"
)]
// ANCHOR: example
//! Finds a button by its label and clicks it with `egui_kittest`.
//!
//! `egui_kittest` runs the UI headless, like the previous recipe, and adds
//! queries based on the accessibility tree (AccessKit), so a test reads like
//! a user: "click the button labelled Increment".

use eframe::egui;
use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;

/// Draws a counter with an `Increment` button.
fn counter_ui(ui: &mut egui::Ui, count: &mut u32) {
    ui.label(format!("Count: {count}"));
    if ui.button("Increment").clicked() {
        *count += 1;
    }
}

/// Clicks the button twice and checks the label.
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
}
// ANCHOR_END: example

#[allow(clippy::main_recursion)]
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the egui kittest example through its entry point.
    #[test]
    fn test_egui_kittest_example_entry_point() {
        main();
    }
}
