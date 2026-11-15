//! `#[debug_handler]` rejects a body extractor that is not the last argument.
//!
//! TODO Reviewed ??-??-2026.

use axum::debug_handler;
use axum::extract::State;

#[derive(Clone)]
struct AppState {

    /// Greeting returned by the server.
    greeting: &'static str,
}

#[debug_handler]
async fn bad(body: String, State(state): State<AppState>) -> String {
    format!("{body}{}", state.greeting)
}

/// Provides the entry point required by this compile-fail fixture.
fn main() {}
