//! Runnable entry point for the useful crates embedded example.
/// Provides postcard queue behavior for the useful crates embedded component.
mod postcard_queue;

/// Runs the example demonstrated by this module.
fn main() -> impl std::process::Termination {
    postcard_queue::run()
}
