//! Runnable entry point for the rate limit example.
/// Provides token bucket behavior for the rate limit component.
mod token_bucket;

/// Runs the example demonstrated by this module.
fn main() {
    token_bucket::run();
}
