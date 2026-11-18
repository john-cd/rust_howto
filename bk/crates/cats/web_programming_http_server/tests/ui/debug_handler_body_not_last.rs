//! `#[debug_handler]` rejects a body extractor that is not the last argument.
//!
//! TODO Reviewed ??-??-2026.

use axum::debug_handler;
use axum::extract::State;

/// State for app state operations that tracks greeting.
#[derive(Clone)]
struct AppState {
    /// Greeting returned by the server.
    greeting: &'static str,
}

/// Deliberately places a body-consuming extractor before another extractor to trigger a diagnostic.
#[debug_handler]
async fn bad(body: String, State(state): State<AppState>) -> String {
    format!("{body}{}", state.greeting)
}

/// Provides the entry point required by this compile-fail fixture.
fn main() {}
