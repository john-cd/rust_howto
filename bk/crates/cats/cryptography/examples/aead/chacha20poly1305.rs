#![allow(dead_code)]
// ANCHOR: example
// Trait that provides in-place stateless AEAD:
use chacha20poly1305::AeadInPlace;
// Algorithm:
use chacha20poly1305::ChaCha20Poly1305;
// Cryptographic key:
use chacha20poly1305::Key;
// A nonce is an arbitrary (often random) number
// that must be used just once in a cryptographic communication.  It is
// used to ensure that the same plaintext encrypted multiple times with the
// same key will produce different ciphertexts.
use chacha20poly1305::Nonce;
// A tag is used to authenticate the ciphertext:
use chacha20poly1305::Tag;
// Trait that provides `generate_nonce`:
use chacha20poly1305::aead::AeadCore;
// Trait that provides `generate_key`, etc:
use chacha20poly1305::aead::KeyInit;
// A random number generator that retrieves randomness from the operating
// system:
use chacha20poly1305::aead::OsRng;
// Trait for securely erasing values from memory:
use zeroize::Zeroize;

// This example demonstrates how to use the ChaCha20Poly1305 AEAD cipher
// to encrypt and decrypt data, with and without associated data.
//
// An Authenticated Encryption with Associated Data (AEAD) cipher,
// based on the ChaCha20 stream cipher and Poly1305 universal hash function,
// ChaCha20Poly1305 is notable for being simple and fast when implemented in
// pure software.

fn encrypt_then_decrypt() {
    // Example plaintext to encrypt.
    let mut plaintext: Vec<u8> = b"This is a secret message.".to_vec();

    // Create a key, using a cryptographically secure Random Number Generator
    let mut key: Key = ChaCha20Poly1305::generate_key(&mut OsRng);

    // Generate a nonce.
    // Nonces MUST be unique for each encryption with the same key.
    let mut nonce: Nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

    let mut ciphertext = plaintext.to_vec();
    let tag: Tag;

    // --- Encryption ---
    {
        // Create an instance of the cipher
        let cipher = ChaCha20Poly1305::new(&key);

        // Encrypt in-place, replacing the plaintext contents with ciphertext,
        // and returning the authentication tag.
        // In-place encryption eliminates `alloc` requirements.
        tag = cipher
            .encrypt_in_place_detached(&nonce, b"", &mut ciphertext)
            .unwrap();

        println!("Ciphertext: {:x?}", ciphertext);
        println!("Tag: {:x?}", tag);
    }
    // --- Decryption ---
    {
        // Create a cipher instance for decryption
        let cipher_dec = ChaCha20Poly1305::new(&key);

        // Decrypt the message in-place, returning an error in the event
        // the provided authentication tag does not match the given ciphertext
        // (i.e. ciphertext is modified/unauthentic)
        let mut decrypted_plaintext = ciphertext.clone();
        let decrypted_result = cipher_dec.decrypt_in_place_detached(
            &nonce,
            b"",                      // No associated_data in this example
            &mut decrypted_plaintext, // Buffer
            &tag,
        );

        match decrypted_result {
            Ok(_) => {
                println!(
                    "Decrypted plaintext: {}",
                    String::from_utf8_lossy(&decrypted_plaintext)
                );
            }
            Err(e) => {
                eprintln!("Decryption error: {}", e);
            }
        }

        // Check if decryption was successful
        assert_eq!(decrypted_plaintext, plaintext);
    }

    // --- Example using an incorrect tag ---
    {
        let mut bad_tag = tag;
        bad_tag[0] ^= 1; // Corrupt the tag

        let cipher_bad = ChaCha20Poly1305::new(&key);
        let mut decrypted_plaintext_bad_tag = ciphertext.clone();
        let decrypted_result_bad_tag = cipher_bad.decrypt_in_place_detached(
            &nonce,
            b"",
            &mut decrypted_plaintext_bad_tag,
            &bad_tag,
        );

        match decrypted_result_bad_tag {
            Ok(_) => {
                println!(
                    "Decryption with bad tag succeeded (this should not happen!)"
                );
            }
            Err(e) => {
                println!("Decryption with bad tag failed as expected: {}", e);
                // This is the expected behavior
            }
        }
    }

    // Fill all sensitive fields with zeroes
    plaintext.zeroize();
    key.zeroize();
    nonce.zeroize();
    ciphertext.zeroize();
}

// --- Example with associated data (AAD) ---
fn aad() {
    let mut plaintext = Vec::from(b"This is a secret message.");
    let aad = b"Associated Data";

    // Create an instance of the cipher (with a new key)
    let mut key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let cipher_aad = ChaCha20Poly1305::new(&key);

    let mut nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

    let mut ciphertext_aad = plaintext.to_vec();
    let tag_aad = cipher_aad
        .encrypt_in_place_detached(&nonce, aad, &mut ciphertext_aad)
        .unwrap();

    println!("Ciphertext with AAD: {:x?}", ciphertext_aad);
    println!("Tag with AAD: {:x?}", tag_aad);

    let mut decrypted_plaintext_aad = ciphertext_aad.clone();
    let decrypted_result_aad = cipher_aad.decrypt_in_place_detached(
        &nonce,
        aad,
        &mut decrypted_plaintext_aad,
        &tag_aad,
    );

    match decrypted_result_aad {
        Ok(_) => {
            println!(
                "Decrypted plaintext with AAD: {}",
                String::from_utf8_lossy(&decrypted_plaintext_aad)
            );
        }
        Err(e) => {
            eprintln!("Decryption error with AAD: {}", e);
        }
    }

    plaintext.zeroize();
    key.zeroize();
    nonce.zeroize();
    ciphertext_aad.zeroize();
}

fn main() {
    encrypt_then_decrypt();
    aad();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
