// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Bringing a function into scope with `use`.

/// This module represents the front of the house in a restaurant.
mod front_of_house {
    /// This module handles hosting duties within the front of the house.
    pub mod hosting {
        /// Adds a customer to the waitlist.
        pub fn add_to_waitlist() -> &'static str {
            "Add to waitlist."
        }
    }
}

// Bring the hosting module in scope.
use front_of_house::hosting;

/// Calls a function through a shortened path.
fn eat_at_restaurant() -> &'static str {
    // We can now access the function within the `hosting` module,
    // without writing the whole path e.g.
    // `front_of_house::hosting::add_to_waitlist()`.
    //
    // IDIOM: import the module, not the function itself.
    hosting::add_to_waitlist()
}

/// Runs the use2 example.
fn main() {
    println!("{}", eat_at_restaurant());
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }

    #[test]
    fn test_shortened_path_calls_same_function() {
        assert_eq!(eat_at_restaurant(), "Add to waitlist.");
        assert_eq!(
            eat_at_restaurant(),
            front_of_house::hosting::add_to_waitlist()
        );
    }
}
