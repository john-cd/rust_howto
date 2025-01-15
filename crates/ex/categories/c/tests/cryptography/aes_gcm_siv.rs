// // ANCHOR: example
// // ANCHOR_END: example

// use aes_gcm_siv::{
//     aead::{Aead, KeyInit, Payload},
//     Aes256GcmSiv, Key, Nonce,
// };
// use rand::Rng;

// fn main() {
//     // Generate a random 256-bit key
//     let key = Key::<Aes256GcmSiv>::generate(&mut rand::thread_rng());

//     // Generate a random 96-bit nonce
//     let nonce = Nonce::from_slice(b"unique nonce");

//     let cipher = Aes256GcmSiv::new(&key);

//     let plaintext = b"Secret message";
//     let ciphertext = cipher
//         .encrypt(nonce, Payload::from(plaintext.as_ref()))
//         .expect("encryption failure!");

//     let decrypted_ciphertext = cipher
//         .decrypt(nonce, Payload::from(ciphertext.as_ref()))
//         .expect("decryption failure!");

//     assert_eq!(plaintext, decrypted_ciphertext.as_slice());

//     println!("Ciphertext: {:?}", ciphertext);
//     println!("Decrypted Ciphertext: {:?}", decrypted_ciphertext);
// }

// #[test]
// fn test() {
//     main();
// }
// // [P1](https://github.com/john-cd/rust_howto/issues/688)
