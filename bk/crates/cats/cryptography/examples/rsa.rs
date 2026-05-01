// ANCHOR: example
// Operating System's Random Number Generator.
use rand::rngs::OsRng;
// Encryption using PKCS#1 v1.5 padding.
use rsa::Pkcs1v15Encrypt;
// Private and Public keys:
use rsa::RsaPrivateKey;
use rsa::RsaPublicKey;

// RSA encryption and decryption.
//
// In this example, we:
// - generate a 2048-bit RSA private key using `RsaPrivateKey::new`.
// - derive the corresponding public key using `RsaPublicKey::from`.
// - encrypt a message to be encrypted using the public key with
// `PaddingScheme::new_pkcs1v15_encrypt`.
// - decrypt the message using the private key with the same padding scheme.
//
// The example is based on the rsa crate documentation.
// Read the security section of <https://docs.rs/rsa>.

fn main() {
    // Define the message to be encrypted:
    let message = b"Secret message";
    println!(
        "Original message: {:?}",
        String::from_utf8(message.to_vec()).unwrap()
    );

    // Generate a 2048-bit private key:
    let mut rng = OsRng;
    let bits = 2048;
    let private_key =
        RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");

    // Derive the public key from the private key:
    let public_key = RsaPublicKey::from(&private_key);

    // Encrypt the message using the public key:
    let encrypted_message = public_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, &message[..])
        .expect("failed to encrypt");

    println!("Encrypted message: {encrypted_message:?}");

    // Decrypt the message using the private key:
    let decrypted_message = private_key
        .decrypt(Pkcs1v15Encrypt, &encrypted_message)
        .expect("failed to decrypt");

    assert_eq!(message.to_vec(), decrypted_message);

    println!(
        "Decrypted message: {:?}",
        String::from_utf8(decrypted_message).unwrap()
    );
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
