//! Runs the localization example.
//!
//! Reviewed 2026-11-17.

/// Loads inline Fluent resources and formats greetings and plural messages.
mod fluent;

/// Runs the localization example.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    fluent::run()
}
