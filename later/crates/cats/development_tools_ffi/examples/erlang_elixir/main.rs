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

/// test their own results).
#[cfg(test)]
mod tests {
    use super::*;

    /// test their own results).
    #[test]
    fn test_main_entry_point() {
        main();
    }
}
