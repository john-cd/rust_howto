#![allow(dead_code)]
// ANCHOR: example

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            // pattern binding to value
            println!("State quarter from {:?}!", state);
            25
        } // if needed, use catchall _ =>
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
