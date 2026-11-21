//! Standalone entry point for the Iced counter example.
//!
//! TODO Reviewed ??-??-2026.

/// Implements the counter state, view, and update logic.
mod iced;

/// Runs the native Iced counter window.
fn main() -> ::iced::Result {
    iced::run()
}
