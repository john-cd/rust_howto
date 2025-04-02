// ANCHOR: example

// In `Cargo.toml`
// rand = { version = "0.9.0", features = [ "thread_rng" ] }

/// This example demonstrates generating random numbers of various types using
/// the `rand` crate.
fn main() {
    // Generate a random value using the thread-local random number generator.
    // `random()` is a shorthand for `rng().random()`
    let n1: u8 = rand::random();
    println!("Random u8: {}", n1);
    // With the "turbofish" notation:
    println!("Random u16: {}", rand::random::<u16>());
    println!("Random u32: {}", rand::random::<u32>());
    println!("Random i32: {}", rand::random::<i32>());
    println!("Random float: {}", rand::random::<f64>());
    // Generate a boolean
    if rand::random() {
        println!("Lucky!");
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
