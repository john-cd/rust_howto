// ANCHOR: example
/// This module represents the front of the house in a restaurant.
mod front_of_house {
    /// This module handles hosting duties within the front of the house.
    pub mod hosting {
        /// Adds a customer to the waitlist.
        pub fn add_to_waitlist() {
            println!("Add to waitlist.");
        }
    }
}

// Bring the hosting module in scope.
use front_of_house::hosting;

fn eat_at_restaurant() {
    // We can now access the function within the `hosting` module,
    // without writing the whole path e.g.
    // `front_of_house::hosting::add_to_waitlist()`.
    //
    // IDIOM: we imported the module, not the function itself.
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
