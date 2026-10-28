// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Fast pseudo-random number generation with `fastrand`.
//!
//! ### Advantages of `fastrand`
//! - **Performance & Size**: Extremely fast execution with minimal dependency
//!   overhead and small compiled binary size.
//! - **Simplicity**: Direct, easy-to-use API for standard random primitives,
//!   sampling, shuffling, and range generation.
//! - **Thread Safety & Custom Rng**: Offers a global thread-local generator as
//!   well as custom-seeded `Rng` instances.
//!
//! ### Limitations & Warnings
//! - **Not Cryptographically Secure**: Uses PCG-XSH-RR, which is predictable.
//!   Do not use `fastrand` for security-sensitive operations such as key
//!   generation, nonce creation, or password hashing. Use `rand` or `getrandom`
//!   instead for cryptographic requirements.
//!
//! BEWARE: `fastrand` is NOT cryptographically secure.

use std::iter::repeat_with;

use fastrand::Rng;

/// Generates random booleans, numbers and strings with `fastrand`, then samples
/// and shuffles values.
fn main() {
    // Seed this independent generator to get reproducible results on every run:
    let mut rng = Rng::with_seed(42);
    // Generate a boolean, number, digit or character from this generator:
    println!("Boolean: {} ", rng.bool());
    // Generate a random u32:
    println!("u32: {} ", rng.u32(..100));
    // Generate a random `Vec` or `String`:
    let mut v: Vec<i32> = repeat_with(|| rng.i32(..)).take(5).collect();
    println!("Vector: {v:?}");
    let s: String = repeat_with(|| rng.alphanumeric()).take(10).collect();
    println!("String: {s}");
    // Sample values from an array or range:
    println!(
        "Two samples from the 0..20 interval: {:?}",
        rng.choose_multiple(0..20, 2)
    );
    // Choose a random element in a vector or array:
    let i = rng.usize(..v.len());
    let elem = v[i];
    println!("Random element: {elem}");
    // Shuffle an array:
    rng.shuffle(&mut v);
    println!("Shuffled vector: {v:?}");
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fastrand_example_entry_point() {
        main();
    }
}
