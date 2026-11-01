// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Recognizes US states by name or by abbreviation with `us-state-info`.
//!
//! State names and two-letter postal abbreviations (CA, NY) are both common in
//! data. A lookup table normalizes either into one form, so forms and datasets
//! can be matched.

use us_state_info::State;

/// Parses a state name or a two-letter abbreviation.
///
/// The match is case-insensitive and ignores spaces.
fn parse_state(text: &str) -> Option<State> {
    // `TryFrom<String>` is implemented. The error type does not implement
    // `std::error::Error`, so convert it to an `Option`.
    State::try_from(text.trim().to_owned()).ok()
}

/// Checks that names and abbreviations map to the same state.
fn main() {
    assert_eq!(parse_state("CA"), Some(State::California));
    assert_eq!(parse_state("California"), parse_state("ca"));
    assert_eq!(parse_state("new york"), Some(State::NewYork));
    // An unknown name is rejected.
    assert_eq!(parse_state("Atlantis"), None);

    if let Some(state) = parse_state("tx") {
        println!("{state} is {}", state.abbreviation());
    }
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the us states example through its entry point.
    #[test]
    fn test_us_states_example_entry_point() {
        main();
    }

    #[test]
    fn rejects_empty_text() {
        assert_eq!(parse_state(""), None);
    }
}
