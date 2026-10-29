//! Runs the Erlang and Elixir interoperability example.
//!
//! TODO Reviewed ??-??-2026.

mod rustler;

/// Runs the erlang elixir example.
fn main() {
    rustler::run();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point (output-only: the modules
    /// test their own results).
    #[test]
    fn test_main_entry_point() {
        main();
    }
}
