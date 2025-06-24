#![allow(dead_code)]
// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example

// use ed25519::Keypair;
// use ed25519::PublicKey;
// use ed25519::SecretKey;
// use ed25519::Signature;
// use rand_core::OsRng;

// // Edwards Digital Signature Algorithm (EdDSA) over Curve25519 (as specified
// in // RFC 8032) support library providing signature type definitions and
// PKCS#8 // private key decoding/encoding support

// fn main() {
//     // 1. Generate a keypair
//     let mut rng = OsRng;
//     let keypair: Keypair = Keypair::generate(&mut rng);

//     // 2. Extract public and secret keys
//     let public_key: PublicKey = keypair.public;
//     let secret_key: SecretKey = keypair.secret;

//     // 3. Create a message to sign
//     let message: &[u8] = b"This is a message to be signed.";

//     // 4. Sign the message
//     let signature: Signature = secret_key.sign(message);

//     // 5. Verify the signature
//     let is_valid: bool = public_key.verify(message, &signature);

//     // 6. Print the results (as byte arrays)
//     println!("Public Key: {:?}", public_key.to_bytes());
//     println!("Secret Key: (Keep this SECRET!)"); // Seriously, don't print
// this in real code!     println!("Signature: {:?}", signature.to_bytes());
//     println!("Signature is valid: {}", is_valid);

//     // 7. Demonstrate an invalid signature (for testing)
//     let different_message: &[u8] = b"A different message.";
//     let invalid_signature: Signature = secret_key.sign(different_message);
//     let is_invalid: bool = public_key.verify(message, &invalid_signature);
//     println!("Invalid signature test: {}", is_invalid);

//     // Example of using from_bytes (error handling included):
//     let public_key_bytes = public_key.to_bytes();
//     let recovered_public_key = PublicKey::from_bytes(&public_key_bytes);
//     match recovered_public_key {
//         Ok(pk) => println!("Recovered public key: {:?}", pk.to_bytes()),
//         Err(e) => println!("Error recovering public key: {}", e),
//     }

//     let signature_bytes = signature.to_bytes();
//     let recovered_signature = Signature::from_bytes(&signature_bytes);
//     match recovered_signature {
//         Ok(sig) => println!("Recovered signature: {:?}", sig.to_bytes()),
//         Err(e) => println!("Error recovering signature: {}", e),
//     }
// }

// #[test]
// fn test() {
//     main();
// }
// // [finish NOW](https://github.com/john-cd/rust_howto/issues/1084)
