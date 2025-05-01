#![allow(dead_code)]
// ANCHOR: example
//! This example defines different types of US coins using an `enum` and then
//! uses a `match` expression to determine the value of a given coin in cents,
//! demonstrating how `match` can handle different `enum` variants and extract
//! data associated with them.

/// Define an `enum`.
/// Each variant represents a type of US coin.
enum Coin {
    Penny,
    Nickel,
    Dime,
    /// The `Quarter` is a tuple variant.
    /// Its field stores the name of a US State.
    Quarter(String),
}

/// Returns the value in cents of a given coin.
fn value_in_cents(coin: Coin) -> u8 {
    // The match expression takes a value and compares it against a series of
    // patterns. Each pattern represents a possible structure or value that the
    // input value might have. It's similar to a 'switch' statement in other
    // languages, but it's far more versatile.
    match coin {
        // If the coin is a Penny, return 1 cent.
        Coin::Penny => 1,
        // If the coin is a Nickel, return 5 cents.
        Coin::Nickel => 5,
        // If the coin is a Dime, return 10 cents.
        Coin::Dime => 10,
        // If the coin is a Quarter, which contains a state string, the `state` identifier is assigned the value of the field in the enum variant.
        Coin::Quarter(state) => {
            // The above is a pattern binding.
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
