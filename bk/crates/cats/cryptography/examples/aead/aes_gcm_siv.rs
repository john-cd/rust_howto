#![allow(dead_code)]
#![allow(deprecated)] // TODO Nonce::from_slice
// ANCHOR: example
use aes_gcm_siv::Aes256GcmSiv;
use aes_gcm_siv::Key;
use aes_gcm_siv::Nonce;
use aes_gcm_siv::aead::Aead;
use aes_gcm_siv::aead::KeyInit;
use aes_gcm_siv::aead::OsRng;
use aes_gcm_siv::aead::Payload;

/// This example demonstrates how to use the `aes-gcm-siv` crate to encrypt and
/// decrypt data using the AES-GCM-SIV algorithm (Misuse-Resistant Authenticated
/// Encryption Cipher).
fn main() -> Result<(), aes_gcm_siv::Error> {
    // Generate a random 256-bit key:
    let key: Key<Aes256GcmSiv> = Aes256GcmSiv::generate_key(&mut OsRng);

    // Generate a random 96-bit nonce.
    // A nonce is an arbitrary (often random) number that can be used just once
    // in a cryptographic communication.
    let nonce = Nonce::from_slice(b"unique nonce");

    let cipher: Aes256GcmSiv = Aes256GcmSiv::new(&key);

    // Encrypt the given plaintext payload, and return the resulting ciphertext
    // as a vector of bytes.
    let plaintext = b"Secret message";
    let associated_data = b"Header";
    let ciphertext: Vec<u8> = cipher.encrypt(
        nonce,
        Payload {
            msg: plaintext,
            aad: associated_data, // AD stays unencrypted...
        },
    )?;
    println!("Ciphertext: {ciphertext:?}");

    // Decrypt.
    // Failure to pass the same AD that was used during encryption will cause
    // decryption to fail:
    let decrypted_ciphertext = cipher.decrypt(
        nonce,
        Payload {
            msg: ciphertext.as_ref(),
            aad: associated_data,
        },
    )?;
    println!(
        "Decrypted Ciphertext: {}",
        String::from_utf8(decrypted_ciphertext.clone()).unwrap()
    );

    assert_eq!(plaintext, decrypted_ciphertext.as_slice());
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main().map_err(|_| anyhow::anyhow!("Failed encryption or decryption"))?;
    Ok(())
}
