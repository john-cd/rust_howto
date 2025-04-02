// ANCHOR: example

/// Generates a random password of a specified length using a predefined
/// character set.
fn main() {
    // Define the character set to choose from.
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";

    // Define the length of the password.
    const PASSWORD_LEN: usize = 30;

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rand::random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    println!("{:?}", password);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
