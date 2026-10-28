// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates how to use the `aes-gcm-siv` crate to encrypt and
//! decrypt data using the AES-GCM-SIV algorithm (Misuse-Resistant Authenticated
//! Encryption Cipher).

// This example demonstrates AES-GCM-SIV encryption and decryption with the
// `aes-gcm-siv` crate.
use aes_gcm_siv::Aes256GcmSiv;
use aes_gcm_siv::Key;
use aes_gcm_siv::Nonce;
use aes_gcm_siv::aead::Aead as _;
use aes_gcm_siv::aead::Generate as _;
use aes_gcm_siv::aead::KeyInit as _;
use aes_gcm_siv::aead::Payload;
use rand::rngs::SysRng;

/// This example demonstrates how to use the `aes-gcm-siv` crate to encrypt and
/// decrypt data using the AES-GCM-SIV algorithm (Misuse-Resistant Authenticated
/// Encryption Cipher).
fn main() -> anyhow::Result<()> {
    // Generate a random 256-bit key:
    let key: Key<Aes256GcmSiv> =
        Key::<Aes256GcmSiv>::try_generate_from_rng(&mut SysRng)?;

    // Use a fixed nonce for this one-message demonstration. A production
    // protocol should follow AES-GCM-SIV's nonce and usage limits.
    let nonce = Nonce::from(*b"unique nonce");

    let cipher: Aes256GcmSiv = Aes256GcmSiv::new(&key);

    // Encrypt the given plaintext payload, and return the resulting ciphertext
    // as a vector of bytes.
    let plaintext = b"Secret message";
    let associated_data = b"Header";
    let ciphertext: Vec<u8> = cipher.encrypt(
        &nonce,
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
        &nonce,
        Payload {
            msg: ciphertext.as_ref(),
            aad: associated_data,
        },
    )?;
    println!(
        "Decrypted Ciphertext: {}",
        String::from_utf8(decrypted_ciphertext.clone())
            .expect("sample plaintext should be valid UTF-8")
    );

    assert_eq!(plaintext, decrypted_ciphertext.as_slice());
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the aes gcm siv example's main path succeeds.
    /// Runs the aes gcm siv example through its entry point.
    #[test]
    fn test_aes_gcm_siv_example_entry_point() -> anyhow::Result<()> {
        main()
            .map_err(|_| anyhow::anyhow!("Failed encryption or decryption"))?;
        Ok(())
    }
}
