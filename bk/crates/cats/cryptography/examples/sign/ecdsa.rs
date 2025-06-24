#![allow(dead_code)]
// // ANCHOR: example
// // Elliptic Curve Digital Signature Algorithm (ECDSA) as specified in FIPS
// 186-4 // (Digital Signature Standard). https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm
// // - A pair of private and public keys are created: data encrypted with
// either //   key can only be decrypted with the other.
// // - A signing entity that published their public key can generate an
// encrypted //   signature
// //  using their private key, and a verifier can assert the source if it is
// // decrypted correctly using the declared public key.

// // The `ecdsa` crate crate provides generic ECDSA support, which can be used
// // with the `k256`, `p256`, `p384` crates. For this example, add to your
// // `Cargo.toml`:
// // p256 = { version = "0.13.2" } # or latest

// use p256::SecretKey;
// use p256::ecdsa::Signature;
// use p256::ecdsa::SigningKey;
// use p256::ecdsa::VerifyingKey;
// use p256::ecdsa::signature::Signer;
// use p256::ecdsa::signature::Verifier;
// // An interface over the operating-system's random data source
// use p256::elliptic_curve::rand_core::OsRng;

// // Serialize a private key object to a PKCS#8 encoded document.
// // (Public-Key Cryptography Standards (PKCS) #8)
// use p256::pkcs8::EncodePrivateKey;

// fn main() {
//     // 1. Generate a random secret key
//     let secret_key = SecretKey::random(&mut OsRng);

//     // 2. Derive public key
//     let public_key = PublicKey::from(&secret_key);

//     // 3. Encode the secret key to PKCS#8 format (for storage/transmission)
//     // PKCS#8 is a format for cryptographic private keys, often containing
// pairs of private and public keys.     let private_key_der =
// secret_key.to_pkcs8()?;     // Get it in PEM format. The PEM text encoding is
// a Base64 representation.     let private_key_pem = private_key_der.to_pem()?;
//     // In a real world application, never print or store private keys in
// plain     // text!
//     println!("Private Key (PEM):\n{}\n", private_key_pem);

//     // 4. Create a message to sign
//     let message = b"Hello, world!";

//     // 5. Sign the message using the secret key
//     let signature: Signature = secret_key.ecdsa_sign(message);
//     // Print the signature in hex format
//     println!("Signature: {:x?}", signature);

//     // 6. Verify the signature using the public key
//     assert!(public_key.verify(message, &signature).is_ok());
//     println!("Message signature verified successfully");
// }
// // ANCHOR_END: example

// #[test]
// fn test() {
//     main();
// }
// // [finish](https://github.com/john-cd/rust_howto/issues/693)
