//! Runnable entry point for the retry example.
/// Provides exponential backoff behavior for the retry component.
mod exponential_backoff;

/// Runs the example demonstrated by this module.
fn main() -> impl std::process::Termination {
    exponential_backoff::run()
}
