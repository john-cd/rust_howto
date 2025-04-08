// ANCHOR: example
// We bring the hosting module in scope.
use front_of_house::hosting;

/// This module represents the front of the house in a restaurant.
mod front_of_house {
    /// This module handles hosting duties.
    pub mod hosting {
        /// Adds a customer to the waitlist.
        pub fn add_to_waitlist() {
            println!("add_to_waitlist");
        }
    }
}

fn eat_at_restaurant() {
    // We can now access the function within the hosting module
    // without specifying the whole path.
    hosting::add_to_waitlist();
}

fn main() {
    eat_at_restaurant();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
