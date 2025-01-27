// // ANCHOR: example
// use chacha20poly1305::ChaCha20Poly1305;
// use chacha20poly1305::Key;
// use chacha20poly1305::aead::Aead;
// use chacha20poly1305::aead::AeadCore;
// use chacha20poly1305::aead::KeyInit;
// use chacha20poly1305::aead::Nonce;

// fn main() {
//     // Example plaintext
//     let plaintext = b"This is a secret message.";

//     // Generate a random key. In real applications, use a cryptographically
//     // secure RNG.
//     let key = Key::from_slice(b"an example very very secret key."); // 32
// bytes

//     // Generate a random nonce. Nonces MUST be unique for each encryption
// with     // the same key.
//     let nonce = Nonce::from_slice(b"unique nonce"); // 12 bytes

//     // Create a cipher instance
//     let cipher = ChaCha20Poly1305::new(key);

//     // Encrypt the plaintext. `encrypt_in_place_detached` returns the
//     // authentication tag.
//     let mut ciphertext = plaintext.to_vec();
//     let tag = cipher
//         .encrypt_in_place_detached(nonce, b"", &mut ciphertext)
//         .unwrap();

//     println!("Ciphertext: {:x?}", ciphertext);
//     println!("Tag: {:x?}", tag);

//     // --- Decryption ---

//     // Create a cipher instance for decryption (same key)
//     let cipher_dec = ChaCha20Poly1305::new(key);

//     // Decrypt the ciphertext. `decrypt_in_place_detached` verifies the tag.
//     let mut decrypted_plaintext = ciphertext.clone();
//     let decrypted_result = cipher_dec.decrypt_in_place_detached(
//         nonce,
//         b"",
//         &mut decrypted_plaintext,
//         &tag,
//     );

//     match decrypted_result {
//         Ok(_) => {
//             println!(
//                 "Decrypted plaintext: {}",
//                 String::from_utf8_lossy(&decrypted_plaintext)
//             );
//             assert_eq!(&decrypted_plaintext, plaintext); // Check if
// decryption was successful         }
//         Err(e) => {
//             eprintln!("Decryption error: {}", e);
//         }
//     }

//     // --- Example with associated data (AAD) ---
//     let aad = b"Associated Data";

//     let mut ciphertext_aad = plaintext.to_vec();
//     let tag_aad = cipher
//         .encrypt_in_place_detached(nonce, aad, &mut ciphertext_aad)
//         .unwrap();

//     println!("Ciphertext with AAD: {:x?}", ciphertext_aad);
//     println!("Tag with AAD: {:x?}", tag_aad);

//     let mut decrypted_plaintext_aad = ciphertext_aad.clone();
//     let decrypted_result_aad = cipher_dec.decrypt_in_place_detached(
//         nonce,
//         aad,
//         &mut decrypted_plaintext_aad,
//         &tag_aad,
//     );

//     match decrypted_result_aad {
//         Ok(_) => {
//             println!(
//                 "Decrypted plaintext with AAD: {}",
//                 String::from_utf8_lossy(&decrypted_plaintext_aad)
//             );
//         }
//         Err(e) => {
//             eprintln!("Decryption error with AAD: {}", e);
//         }
//     }

//     // Example of incorrect tag
//     let mut bad_tag = tag.clone();
//     bad_tag[0] ^= 1; // Corrupt the tag

//     let mut decrypted_plaintext_bad_tag = ciphertext.clone();
//     let decrypted_result_bad_tag = cipher_dec.decrypt_in_place_detached(
//         nonce,
//         b"",
//         &mut decrypted_plaintext_bad_tag,
//         &bad_tag,
//     );

//     match decrypted_result_bad_tag {
//         Ok(_) => {
//             println!(
//                 "Decryption with bad tag succeeded (this should not happen!)"
//             );
//         }
//         Err(e) => {
//             println!("Decryption with bad tag failed as expected: {}", e); //
// This is the expected behavior         }
//     }
// }
// // ANCHOR_END: example

// #[test]
// fn test() {
//     main();
// }
// [ P1 write; add to markdown](https://github.com/john-cd/rust_howto/issues/1014)
