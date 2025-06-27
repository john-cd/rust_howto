#![allow(dead_code)]
// ANCHOR: example
/// The `if let` syntax allows you to combine `if` and `let`
/// to handle values that match a specific pattern, while ignoring others.
fn main() {
    let config_max: Option<u8> = Some(3u8);

    // If `config_max` is `Some`, bind the inner value to `max`
    if let Some(max) = config_max {
        // `max` is available here.
        println!("The maximum is configured to be {max}.");
    }

    // This is equivalent to the following `match` expression:
    #[allow(clippy::single_match)]
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}."),
        _ => {} // Required because `match` is exhaustive.
    };

    enum Direction {
        Left,
        Right,
    }

    let left = Direction::Left;

    // `if let` can match non-parameterized, Unit-like enum variants.
    // This works even if the enum does not implement `PartialEq`.
    if let Direction::Left = left {
        println!("Going left.");
    } else {
        // You can also add `else` and `else if` clauses to `if let`.
        println!("Going right.");
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
