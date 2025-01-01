// // ANCHOR: example
// use rand::rngs::OsRng;
// use rsa::PaddingScheme;
// use rsa::PublicKey;
// use rsa::RsaPrivateKey;
// use rsa::RsaPublicKey;
// use sha2::Sha256;

// // In this example:

// // We generate a 2048-bit RSA private key using RsaPrivateKey::new.

// // We derive the corresponding public key using RsaPublicKey::from.

// // We define a message to be encrypted.

// // We encrypt the message using the public key with
// // PaddingScheme::new_pkcs1v15_encrypt.

// // We decrypt the message using the private key with the same padding scheme.

// // This code demonstrates how to perform RSA encryption and decryption in
// Rust.

// fn main() {
//     // Generate a 2048-bit private key
//     let mut rng = OsRng;
//     let bits = 2048;
//     let private_key =
//         RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a
// key");

//     // Derive the public key from the private key
//     let public_key = RsaPublicKey::from(&private_key);

//     // Define the message to be encrypted
//     let message = b"Secret message";

//     // Encrypt the message using the public key
//     let padding = PaddingScheme::new_pkcs1v15_encrypt();
//     let encrypted_message = public_key
//         .encrypt(&mut rng, padding, &message[..])
//         .expect("failed to encrypt");

//     // Decrypt the message using the private key
//     let padding = PaddingScheme::new_pkcs1v15_encrypt();
//     let decrypted_message = private_key
//         .decrypt(padding, &encrypted_message)
//         .expect("failed to decrypt");

//     assert_eq!(message.to_vec(), decrypted_message);

//     println!(
//         "Original message: {:?}",
//         String::from_utf8(message.to_vec()).unwrap()
//     );
//     println!("Encrypted message: {:?}", encrypted_message);
//     println!(
//         "Decrypted message: {:?}",
//         String::from_utf8(decrypted_message).unwrap()
//     );
// }
// // ANCHOR_END: example

// #[test]
// #[ignore = "not yet implemented"]
// fn test() {
//     main();
// }
// // [P1](https://github.com/john-cd/rust_howto/issues/699)
