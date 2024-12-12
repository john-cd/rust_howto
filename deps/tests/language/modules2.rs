// ANCHOR: example

// We bring the hosting module in scope...
use front_of_house::hosting;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add_to_waitlist");
        }
    }
}

fn eat_at_restaurant() {
    // ...then we access the function within
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
