#![allow(dead_code)]
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

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Hello egui",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

#[derive(Default)]
struct MyApp {
    counter: i32,
}

// Simple GUI application with a button to increment a counter.
impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("Hello egui!");
            if ui.button("Increment counter").clicked() {
                self.counter += 1;
            }
            ui.label(format!("Counter: {}", self.counter));
        });
    }
}
// ANCHOR_END: example

pub fn run() -> Result<(), eframe::Error> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "requires interactive example runtime"]
    fn test_main() {
        main().unwrap();
    }
}

// Hard to test. Initializing the event loop outside of the main thread is a
// significant cross-platform compatibility hazard.

// [finish](https://github.com/john-cd/rust_howto/issues/776) verify the code manually
// thread '<unnamed>' panicked at
// /usr/local/cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zbus-4.4.0/
// src/abstractions/executor.rs:189:27: there is no reactor running, must be
// called from the context of a Tokio 1.x runtime
