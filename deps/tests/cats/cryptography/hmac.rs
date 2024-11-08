// ANCHOR: example
use ring::error::Unspecified;
use ring::hmac;
use ring::rand;
use ring::rand::SecureRandom;

fn main() -> Result<(), Unspecified> {
    let mut key_value = [0u8; 48];
    let rng = rand::SystemRandom::new();
    rng.fill(&mut key_value)?;
    let key = hmac::Key::new(hmac::HMAC_SHA256, &key_value);

    let message = "Legitimate and important message.";
    let signature = hmac::sign(&key, message.as_bytes());
    hmac::verify(&key, message.as_bytes(), signature.as_ref())?;

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() {
    println!("{:?}", main());
}
