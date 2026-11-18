//! Runnable entry point for the task automation example.
/// Provides scheduled command behavior for the task automation component.
mod scheduled_command;

/// Runs the example demonstrated by this module.
fn main() -> impl std::process::Termination {
    scheduled_command::run()
}
