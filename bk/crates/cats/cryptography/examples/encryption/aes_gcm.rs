// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Authenticated encryption and decryption with AES-256-GCM.

// Algorithm:
use aes_gcm::Aes256Gcm; // Use Aes128Gcm or Aes256Gcm, respectively.
// Cryptokey:
use aes_gcm::Key;
// A 'nonce' is an arbitrary (often random) number
// that must be used just once
// in a cryptographic communication.
// Trait that provides `generate_key`:
use aes_gcm::aead::KeyInit as _;
// The operating-system's random data source:
use aes_gcm::aead::OsRng;
// Trait that provides `encrypt` and `decrypt`:
use aes_gcm::aead::{Aead as _, AeadCore as _};

/// Encrypts a message with a random key and decrypts it again.
fn main() -> anyhow::Result<()> {
    // Generate a random 256-bit key:
    let key: Key<Aes256Gcm> = Aes256Gcm::generate_key(&mut OsRng);

    // Generate a fresh nonce. A production protocol must also coordinate
    // nonce allocation across every writer that shares this key.
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    // Create the cipher...
    let cipher = Aes256Gcm::new(&key);

    let plaintext = b"Secret message";

    // ...that will encrypt the secret message.
    // To authenticate additional Associated Data,
    // apply the same method than in the AES-GCM-SIV example:
    let ciphertext = cipher
        .encrypt(&nonce, plaintext.as_ref())
        .map_err(|_| anyhow::anyhow!("AES-GCM encryption failed"))?;
    println!("Ciphertext: {ciphertext:?}");

    let decrypted_ciphertext = cipher
        .decrypt(&nonce, ciphertext.as_ref())
        .map_err(|_| anyhow::anyhow!("AES-GCM authentication failed"))?;

    assert_eq!(plaintext, decrypted_ciphertext.as_slice());

    println!(
        "Decrypted plaintext: {}",
        String::from_utf8(decrypted_ciphertext)?
    );
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aes_gcm_round_trips_and_rejects_tampering() -> anyhow::Result<()> {
        main()?;

        let key: Key<Aes256Gcm> = Aes256Gcm::generate_key(&mut OsRng);
        let cipher = Aes256Gcm::new(&key);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let plaintext = b"authenticated message";
        let mut ciphertext = cipher
            .encrypt(&nonce, plaintext.as_ref())
            .map_err(|_| anyhow::anyhow!("AES-GCM encryption failed"))?;
        let decrypted = cipher
            .decrypt(&nonce, ciphertext.as_ref())
            .map_err(|_| anyhow::anyhow!("AES-GCM authentication failed"))?;
        assert_eq!(decrypted, plaintext);

        ciphertext[0] ^= 1;
        assert!(cipher.decrypt(&nonce, ciphertext.as_ref()).is_err());
        Ok(())
    }
}
