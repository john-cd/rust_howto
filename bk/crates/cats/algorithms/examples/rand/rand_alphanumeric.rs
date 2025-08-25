#![allow(dead_code)]
// ANCHOR: example
use rand::Rng;
use rand::distr::Alphanumeric;
use rand::distr::SampleString;

/// Generate a random string of 20 alphanumeric characters.
fn main() {
    let rand_string: String = rand::rng()
        // Create an iterator that generates values using the `Alphanumeric` distribution:
        .sample_iter(&Alphanumeric)
        // Take the first 20 characters:
        .take(20)
        .map(char::from)
        .collect();
    println!("Randon string: {rand_string}");

    // Or use the `SampleString` trait:
    let rand_string2 = Alphanumeric.sample_string(&mut rand::rng(), 20);
    println!("Another random string: {rand_string2}");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
