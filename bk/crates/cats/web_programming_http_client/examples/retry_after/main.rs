//! Runnable entry point for the retry after example.
/// Provides retry after header behavior for the retry after component.
mod retry_after_header;

/// Runs the example demonstrated by this module.
fn main() -> impl std::process::Termination {
    retry_after_header::run()
}
