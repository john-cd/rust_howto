#![allow(dead_code)]
// ANCHOR: example
/// Define an `enum` that represents different types of US coins.
enum Coin {
    Penny,
    Nickel,
    Dime,
    /// Represents a quarter, which may be associated with a state.
    Quarter(String),
}

/// Returns the value in cents of a given coin.
fn value_in_cents(coin: Coin) -> u8 {
    // The match expression takes a value and compares it against a series of
    // patterns. Each pattern represents a possible structure or value that the
    // input value might have. It's similar to a 'switch' statement in other
    // languages, but it's far more versatile.
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            // The above is a pattern binding.
            // `state` is assigned.
            println!("State quarter from {:?}!", state);
            25
        } // Rust's match expressions are exhaustive. This means that you must cover all possible cases.
          // If needed, you can use a catchall:
          //_ => unreachable!(),
    }
}

fn main() {
    println!("{}", value_in_cents(Coin::Penny));
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
