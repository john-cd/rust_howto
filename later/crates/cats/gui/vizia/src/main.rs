// TODO Reviewed ??-??-2026.

//! Application entry point for vizia.
/// Provides signal counter behavior for the src component.
mod signal_counter;

/// Runs the example demonstrated by this module.
fn main() -> impl std::process::Termination {
    signal_counter::run()
}
