// ANCHOR: example
use ring::error::Unspecified;
use ring::hmac;
use ring::rand;
use ring::rand::SecureRandom;

fn main() -> Result<(), Unspecified> {
    // Create a key
    let mut key_value = [0u8; 48];
    let rng = rand::SystemRandom::new();
    rng.fill(&mut key_value)?;
    let key = hmac::Key::new(hmac::HMAC_SHA256, &key_value);

    // Sign a message
    let message = "Legitimate and important message.";
    let signature = hmac::sign(&key, message.as_bytes());

    // Calculates the HMAC of data using the signing key key,
    // and verifies whether the resultant value equals the signature.
    hmac::verify(&key, message.as_bytes(), signature.as_ref())?;
    println!("Message verified.");
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() {
    println!("{:?}", main());
}
// TODO
