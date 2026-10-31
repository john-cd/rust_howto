// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Authenticated encryption with ChaCha20-Poly1305, with and without associated
//! data.
//!
//! ChaCha20-Poly1305 is a modern authenticated cipher, fast even without
//! hardware AES support. Authenticated means decryption fails if the data was
//! tampered with. Associated data is extra information, such as a header, that
//! is authenticated but not encrypted. Never reuse a nonce with the same key.

// Trait that provides in-place stateless AEAD:
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
use chacha20poly1305::aead::AeadInOut as _;
use chacha20poly1305::aead::Generate as _;
// Trait that provides `generate_nonce`:
use chacha20poly1305::aead::KeyInit as _;
use rand::rngs::SysRng;
// Trait for securely erasing values from memory:
use zeroize::Zeroize as _;

// This example demonstrates how to use the ChaCha20Poly1305 AEAD cipher
// to encrypt and decrypt data, with and without associated data.
//
// An "Authenticated Encryption with Associated Data" (AEAD) cipher,
// based on the "ChaCha20" stream cipher and "Poly1305" universal hash function,
// "ChaCha20Poly1305" is notable for being simple and fast when implemented in
// pure software.

/// Encrypts a message and decrypts it again.
fn encrypt_then_decrypt() -> anyhow::Result<()> {
    // Example plaintext to encrypt.
    let mut plaintext: Vec<u8> = b"This is a secret message.".to_vec();

    // Create a key, using a cryptographically secure Random Number Generator:
    let mut rng = SysRng;
    let mut key: Key = Key::try_generate_from_rng(&mut rng)?;

    // Generate a nonce.
    // Nonces MUST be unique for each encryption with the same key.
    let mut nonce: Nonce = Nonce::try_generate_from_rng(&mut rng)?;

    let mut ciphertext = plaintext.clone();
    let tag: Tag;

    // --- Encryption ---
    {
        // Create an instance of the cipher:
        let cipher = ChaCha20Poly1305::new(&key);

        // Encrypt in-place, replacing the plaintext contents with ciphertext,
        // and returning the authentication tag.
        // In-place encryption eliminates `alloc` requirements.
        tag = cipher
            .encrypt_inout_detached(
                &nonce,
                b"",
                ciphertext.as_mut_slice().into(),
            )
            .map_err(|_| {
                anyhow::anyhow!("ChaCha20-Poly1305 encryption failed")
            })?;

        println!("Ciphertext: {ciphertext:x?}");
        println!("Tag: {tag:x?}");
    }
    // --- Decryption ---
    {
        // Create a cipher instance for decryption:
        let cipher_dec = ChaCha20Poly1305::new(&key);

        // Decrypt the message in-place, returning an error in the event
        // the provided authentication tag does not match the given ciphertext
        // (i.e. ciphertext is modified/unauthentic):
        let mut decrypted_plaintext = ciphertext.clone();
        let decrypted_result = cipher_dec.decrypt_inout_detached(
            &nonce,
            b"", // No associated_data in this example
            decrypted_plaintext.as_mut_slice().into(), // Buffer
            &tag,
        );

        decrypted_result.map_err(|_| {
            anyhow::anyhow!("ChaCha20-Poly1305 authentication failed")
        })?;
        assert_eq!(decrypted_plaintext, plaintext);
    }

    // --- Example using an incorrect tag ---
    {
        let mut bad_tag = tag;
        bad_tag[0] ^= 1; // Corrupt the tag

        let cipher_bad = ChaCha20Poly1305::new(&key);
        let mut decrypted_plaintext_bad_tag = ciphertext.clone();
        let decrypted_result_bad_tag = cipher_bad.decrypt_inout_detached(
            &nonce,
            b"",
            decrypted_plaintext_bad_tag.as_mut_slice().into(),
            &bad_tag,
        );

        assert!(decrypted_result_bad_tag.is_err());
        println!("Decryption with modified tag failed as expected.");
    }

    // Fill all sensitive fields with zeroes.
    plaintext.zeroize();
    key.zeroize();
    nonce.zeroize();
    ciphertext.zeroize();
    Ok(())
}

// --- Example with associated data (AAD) ---
/// Encrypts with associated data, which must match when decrypting.
fn aad() -> anyhow::Result<()> {
    let mut plaintext = Vec::from(b"This is a secret message.");
    let aad = b"Associated Data";

    // Create an instance of the cipher (with a new key):
    let mut rng = SysRng;
    let mut key: Key = Key::try_generate_from_rng(&mut rng)?;
    let cipher_aad = ChaCha20Poly1305::new(&key);

    let mut nonce: Nonce = Nonce::try_generate_from_rng(&mut rng)?;

    let mut ciphertext_aad = plaintext.clone();
    let tag_aad = cipher_aad
        .encrypt_inout_detached(
            &nonce,
            aad,
            ciphertext_aad.as_mut_slice().into(),
        )
        .map_err(|_| anyhow::anyhow!("ChaCha20-Poly1305 encryption failed"))?;

    println!("Ciphertext with AAD: {ciphertext_aad:x?}");
    println!("Tag with AAD: {tag_aad:x?}");

    let mut decrypted_plaintext_aad = ciphertext_aad.clone();
    let decrypted_result_aad = cipher_aad.decrypt_inout_detached(
        &nonce,
        aad,
        decrypted_plaintext_aad.as_mut_slice().into(),
        &tag_aad,
    );

    decrypted_result_aad.map_err(|_| {
        anyhow::anyhow!("ChaCha20-Poly1305 authentication failed")
    })?;
    assert_eq!(decrypted_plaintext_aad, plaintext);

    let wrong_aad = b"different associated data";
    let mut tampered_plaintext = ciphertext_aad.clone();
    assert!(
        cipher_aad
            .decrypt_inout_detached(
                &nonce,
                wrong_aad,
                tampered_plaintext.as_mut_slice().into(),
                &tag_aad
            )
            .is_err()
    );

    plaintext.zeroize();
    key.zeroize();
    nonce.zeroize();
    ciphertext_aad.zeroize();
    Ok(())
}

/// Runs both examples.
fn main() -> anyhow::Result<()> {
    encrypt_then_decrypt()?;
    aad()?;
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies ChaCha20-Poly1305 round trips and rejects altered tags and AAD.
    #[test]
    fn test_chacha20poly1305_round_trip_and_tampering() -> anyhow::Result<()> {
        main()
    }
}
