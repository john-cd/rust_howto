//! Runs the Erlang and Elixir interoperability example.
//!
//! TODO Reviewed ??-??-2026.

/// This module provides the `rustler` implementation used by its parent.
mod rustler;
/// Runs the example.
/// Runs the erlang elixir example.
fn main() {
    rustler::run();
}
