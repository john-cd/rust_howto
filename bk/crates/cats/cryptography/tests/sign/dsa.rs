// ANCHOR: example
use p256::ecdsa::Signature;
use p256::ecdsa::SigningKey;
use p256::ecdsa::VerifyingKey;
use p256::ecdsa::signature::Signer;
use p256::ecdsa::signature::Verifier;
use p256::elliptic_curve::rand_core::OsRng;

fn main() {
    // Generate a signing key
    let signing_key = SigningKey::random(&mut OsRng);

    // Create a verifying key from the signing key
    let verifying_key = VerifyingKey::from(&signing_key);

    // Message to be signed
    let message = b"Hello, world!";

    // Sign the message
    let signature: Signature = signing_key.sign(message);

    // Verify the signature
    assert!(verifying_key.verify(message, &signature).is_ok());
    println!("Message signature verified successfully");

    // Print the signature in hex format
    println!("Signature: {:x?}", signature);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
