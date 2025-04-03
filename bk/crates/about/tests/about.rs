// ANCHOR: example
use rand::Rng;

/// This is a simple example that demonstrates how to use the `rand` crate to
/// generate a random `f64`.
fn main() {
    // Get a thread-local random number generator.
    let mut rng = rand::rng();
    // Generate a random f64.
    println!("Random f64: {}", rng.random::<f64>());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
