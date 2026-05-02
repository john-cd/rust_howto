// ANCHOR: example
use rand::RngExt;

/// This is a simple example that demonstrates how to use the `rand` crate to
/// generate a random `f64`.
fn main() {
    // Get a thread-local random number generator.
    let mut rng = rand::rng();
    // Generate a random `f64`.
    println!("Random f64: {}", rng.random::<f64>());
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        main();
    }
}
