// // ANCHOR: example
// // ANCHOR_END: example

// use aes_gcm::{
//     Aes256Gcm, // Use Aes128Gcm or Aes256Gcm, respectively
//     aead::{Aead, AeadInPlace, Key, KeyInit, Nonce, OsRng, Payload}, /* Or
// `AeadInPlace` for in-place encryption */ };

// fn main() {
//     // Generate a random 256-bit key
//     let key = Key::<Aes256Gcm>::generate(&mut OsRng);

//     // Generate a random 96-bit nonce
//     let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per
// message

//     let cipher = Aes256Gcm::new(&key);

//     let plaintext = b"Secret message";
//     let ciphertext = cipher
//         .encrypt(nonce, plaintext.as_ref())
//         .expect("encryption failure!");

//     let decrypted_ciphertext = cipher
//         .decrypt(nonce, ciphertext.as_ref())
//         .expect("decryption failure!");

//     assert_eq!(plaintext, decrypted_ciphertext.as_slice());

//     println!("Ciphertext: {:?}", ciphertext);
//     println!("Decrypted Ciphertext: {:?}", decrypted_ciphertext);
// }

// #[test]
// fn test() {
//     main();
// }
// // [P1](https://github.com/john-cd/rust_howto/issues/687)
