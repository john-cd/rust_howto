//! Runnable entry point for the hals example.
/// Provides blink output pin behavior for the hals component.
mod blink_output_pin;

/// Runs the example demonstrated by this module.
fn main() -> impl std::process::Termination {
    blink_output_pin::run()
}
