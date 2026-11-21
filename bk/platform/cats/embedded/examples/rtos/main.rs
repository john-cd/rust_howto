// TODO Reviewed ??-??-2026.
//! Runnable entry point for the rtos example.
/// Provides fixed priority schedule behavior for the rtos component.
mod fixed_priority_schedule;

/// Runs the example demonstrated by this module.
fn main() -> impl std::process::Termination {
    fixed_priority_schedule::run()
}
