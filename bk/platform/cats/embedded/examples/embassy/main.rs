//! Runnable entry point for the embassy example.
/// Provides embassy behavior for the embassy component.
mod embassy;

/// Runs the example demonstrated by this module.
fn main() -> impl std::process::Termination {
    embassy::run()
}
