// ANCHOR: example

/// The `if let` syntax allows you to combine `if` and `let`
/// to handle values that match a specific pattern, while ignoring others.
fn main() {
    // `config_max` is an `Option<u8>`
    let config_max = Some(3u8);
    // If `config_max` is `Some`, bind the inner value to `max`
    if let Some(max) = config_max {
        // `max` is available here
        println!("The maximum is configured to be {}", max);
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
