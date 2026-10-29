//! `#[debug_handler]` rejects a body extractor that is not the last argument.

use axum::debug_handler;
use axum::extract::State;

#[derive(Clone)]
struct AppState {
    greeting: &'static str,
}

#[debug_handler]
async fn bad(body: String, State(state): State<AppState>) -> String {
    format!("{body}{}", state.greeting)
}

fn main() {}
