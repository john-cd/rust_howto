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

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
