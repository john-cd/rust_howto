#![allow(dead_code)]
// ANCHOR: example
//! In `Cargo.toml`, add:
//! ```toml
//! rand = { version = "0.9.0" } # Or latest.
//! ```

fn main() {
    // Generate a random value:.
    let n1: u8 = rand::random();
    println!("Random u8: {n1}");

    // Use the "turbofish" notation to specify the type.
    // Integers are uniformly distributed over the range of the type:
    println!("Random u16: {}", rand::random::<u16>());
    println!("Random u32: {}", rand::random::<u32>());

    // Floating point numbers are uniformly distributed from 0 up to but not
    // including 1:
    println!("Random float: {}", rand::random::<f64>());

    // Generate a boolean. Consider the `random_bool` function as well.
    if rand::random() {
        println!("Lucky!");
    }
    // `random()` is a shorthand for `rng().random()`, where `rng()` returns
    // a handle to the local `ThreadRng`, a lazily-initialized thread-local
    // generator.
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
