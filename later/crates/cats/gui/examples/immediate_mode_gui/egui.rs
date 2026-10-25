#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Simple example of a GUI application with `egui`.
//!
//! In `Cargo.toml`, add:
//! ```toml
//! [dependencies]
//! egui = "0.34"
//! eframe = "0.34"
//! ```

use eframe::egui;

/// Opens a native window that runs the app.
fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Counter",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

/// An app with a counter.
#[derive(Default)]
struct MyApp {
    /// The number of clicks on the button.
    counter: i32,
    /// The text shown in the editable text field.
    name: String,
    /// The value selected by the slider.
    amount: f32,
}

impl MyApp {
    /// Draws the app's controls and returns the increment button's rectangle.
    fn show(&mut self, ui: &mut egui::Ui) -> egui::Rect {
        egui::CentralPanel::default()
            .show_inside(ui, |ui| {
                ui.heading("Hello egui!");
                ui.text_edit_singleline(&mut self.name);
                ui.add(
                    egui::Slider::new(&mut self.amount, 0.0..=1.0)
                        .text("Amount"),
                );
                let button = ui.button("Increment counter");
                if button.clicked() {
                    self.counter += 1;
                }
                ui.label(format!("Counter: {}", self.counter));
                button.rect
            })
            .inner
    }
}

impl eframe::App for MyApp {
    /// Draws one frame of the application.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.show(ui);
    }
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() -> Result<(), eframe::Error> {
    main()
}

// eframe's native runner is deliberately covered by the ignored window test.

/// Builds pointer input for the headless UI test.
#[cfg(test)]
fn raw_input(events: Vec<egui::Event>) -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(500.0, 400.0),
        )),
        events,
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the counter button responds to a simulated headless click.
    #[test]
    fn headless_frame_increments_the_counter_on_click() {
        let mut app = MyApp::default();
        let context = egui::Context::default();
        let mut button_rect = egui::Rect::NOTHING;

        let _ = context.run_ui(raw_input(Vec::new()), |ui| {
            button_rect = app.show(ui);
        });

        let position = button_rect.center();
        let _ = context.run_ui(
            raw_input(vec![
                egui::Event::PointerMoved(position),
                egui::Event::PointerButton {
                    pos: position,
                    button: egui::PointerButton::Primary,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                },
            ]),
            |ui| {
                app.show(ui);
            },
        );
        let _ = context.run_ui(
            raw_input(vec![egui::Event::PointerButton {
                pos: position,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            }]),
            |ui| {
                app.show(ui);
            },
        );

        assert_eq!(app.counter, 1);
    }

    /// Keeps launching a native window out of headless test runs.
    #[test]
    #[ignore = "requires an interactive display"]
    fn native_window_runs() -> Result<(), eframe::Error> {
        main()
    }
}
