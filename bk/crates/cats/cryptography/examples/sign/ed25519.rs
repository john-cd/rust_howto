#![allow(dead_code)]
// ANCHOR: example
use ed25519_dalek::{Signer, Verifier, SigningKey, VerifyingKey, Signature};
use rand_core_06::OsRng;

fn main() {
    let mut csprng = OsRng;
    let signing_key: SigningKey = SigningKey::generate(&mut csprng);
    let verifying_key: VerifyingKey = signing_key.verifying_key();

    let message: &[u8] = b"This is a message to be signed.";
    let signature: Signature = signing_key.sign(message);

    let is_valid: bool = verifying_key.verify(message, &signature).is_ok();
    println!("Signature is valid: {}", is_valid);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
