// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Generate a random string of 20 alphanumeric characters.
//!
//! Random strings serve as IDs, temporary names and tokens. A default random
//! generator is fine for IDs, but secrets such as session tokens need a
//! cryptographically secure source.

use rand::RngExt as _;
use rand::distr::Alphanumeric;
use rand::distr::SampleString as _;

/// Generate a random string of 20 alphanumeric characters.
fn main() {
    let rand_string: String = rand::rng()
        // Create an iterator that generates values using the `Alphanumeric`
        // distribution:
        .sample_iter(&Alphanumeric)
        // Take the first 20 characters:
        .take(20)
        .map(char::from)
        .collect();
    println!("Random string: {rand_string}");

    // Or use the `SampleString` trait:
    let rand_string2 = Alphanumeric.sample_string(&mut rand::rng(), 20);
    println!("Another random string: {rand_string2}");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rand_alphanumeric_example_entry_point() {
        main();
    }

    #[test]
    fn sample_iter_yields_alphanumeric_strings_of_the_requested_length() {
        let s: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(20)
            .map(char::from)
            .collect();
        assert_eq!(s.len(), 20);
        assert!(s.chars().all(|c| c.is_ascii_alphanumeric()));
    }

    #[test]
    fn sample_string_yields_alphanumeric_strings_of_the_requested_length() {
        let s = Alphanumeric.sample_string(&mut rand::rng(), 20);
        assert_eq!(s.len(), 20);
        assert!(s.chars().all(|c| c.is_ascii_alphanumeric()));
        assert!(Alphanumeric.sample_string(&mut rand::rng(), 0).is_empty());
    }
}
