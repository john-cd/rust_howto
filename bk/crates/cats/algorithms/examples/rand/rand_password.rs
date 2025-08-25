#![allow(dead_code)]
// ANCHOR: example
use rand::Rng;

fn main() {
    // Define the character set to choose from.
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";

    // Define the length of the password.
    const PASSWORD_LEN: usize = 15;

    // Get handle to the local `ThreadRng`, which implements `rand::CryptoRng`.
    // Read the following about the suitability of `rand` to generate random
    // passwords: <https://docs.rs/rand_distr/latest/rand_distr/struct.Alphanumeric.html#passwords>
    let mut rng = rand::rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    println!("{password:?}");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
