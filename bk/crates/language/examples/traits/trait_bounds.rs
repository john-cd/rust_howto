// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Trait bounds on functions and implementations.

use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

/// `GenericTypeParameter: Trait` is called the "trait bound" syntax.
/// Trait bounds allow you to specify that a generic type must implement a
/// certain trait.
///
/// Here, the `print_hash` function is generic over an unknown
/// type `T`, but requires that `T` implements the `Hash` trait.
fn print_hash<T: Hash>(t: &T) {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    // The `finish` method returns the computed hash.
    println!("The hash is {:x}", hasher.finish());
}

/// This struct represents a pair of values.
///
/// It is generic over the types of the two values.
struct Pair<A, B> {
    /// The first value.
    first: A,
    /// The second value.
    second: B,
}

// Generics and trait bounds make it possible to implement a trait
// conditionally. Here, the `Pair` type implements `Hash` if its components do.
impl<A: Hash, B: Hash> Hash for Pair<A, B> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.first.hash(state);
        self.second.hash(state);
    }
}

/// Hashes a value and a pair.
fn main() {
    let p = Pair {
        first: 1,
        second: "2",
    };
    print_hash(&p);
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the trait bounds example through its entry point.
    #[test]
    fn test_trait_bounds_example_entry_point() {
        main();
    }

    /// Equal values hash equally; different values (almost surely) do not.
    #[test]
    fn test_pair_hash() {
        fn hash_of<T: Hash>(t: &T) -> u64 {
            let mut hasher = DefaultHasher::new();
            t.hash(&mut hasher);
            hasher.finish()
        }
        let p = Pair {
            first: 1,
            second: "2",
        };
        let same = Pair {
            first: 1,
            second: "2",
        };
        let other = Pair {
            first: 2,
            second: "2",
        };
        assert_eq!(hash_of(&p), hash_of(&same));
        assert_ne!(hash_of(&p), hash_of(&other));
        // The pair hashes like its fields in sequence.
        assert_eq!(hash_of(&p), {
            let mut hasher = DefaultHasher::new();
            1.hash(&mut hasher);
            "2".hash(&mut hasher);
            hasher.finish()
        });
        print_hash(&p);
    }
}
