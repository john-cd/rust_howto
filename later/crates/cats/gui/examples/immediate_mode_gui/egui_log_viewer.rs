// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Shows the `tracing` events of an application in a panel with `egui_tracing`.

use eframe_034::egui;
use egui_tracing::EventCollector;
use egui_tracing::Logs;
use tracing_subscriber::prelude::*;

/// Creates a collector and a subscriber that feeds it.
///
/// `EventCollector` is a `tracing_subscriber::Layer`: it stores the events.
/// An application calls `.init()` once to install the subscriber globally;
/// a subscriber returned by this function can also be scoped to a test.
fn new_collector() -> (EventCollector, impl tracing::Subscriber) {
    let collector = EventCollector::default();
    let subscriber = tracing_subscriber::registry().with(collector.clone());
    (collector, subscriber)
}

/// Logs a few events and draws the panel for one frame without a window.
fn run_example() {
    let (collector, subscriber) = new_collector();

    tracing::subscriber::with_default(subscriber, || {
        tracing::info!("application started");
        tracing::warn!(attempts = 3, "retrying");
        tracing::error!("something failed");
    });
    // `events` also collects the events that are still pending.
    assert_eq!(collector.events().len(), 3);

    let ctx = egui::Context::default();
    let screen =
        egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(800.0, 600.0));
    let input = egui::RawInput {
        screen_rect: Some(screen),
        ..Default::default()
    };
    let _output = ctx.run_ui(input, |ui| {
        egui::Panel::bottom("log_panel")
            .resizable(true)
            .show_inside(ui, |ui| {
                // `Logs` is a widget with a filter by level and by target.
                ui.add(Logs::new(collector.clone()));
            });
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.label("Main content");
        });
    });
}
fn main() {
    run_example();
}
// ANCHOR_END: example

pub fn run() {
    run_example();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the egui log viewer example through its entry point.
    #[test]
    fn test_egui_log_viewer_example_entry_point() {
        run_example();
    }

    #[test]
    fn events_outside_the_subscriber_are_not_collected() {
        let (collector, _subscriber) = new_collector();
        tracing::info!("nobody listens");
        assert!(collector.events().is_empty());
    }
}
