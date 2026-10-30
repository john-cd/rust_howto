// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Authenticated encryption and decryption with AES-256-GCM.
//!
//! AES is the standard symmetric cipher, meaning one secret key both encrypts
//! and decrypts. GCM mode adds authentication: decryption fails if the data was
//! tampered with. Each message needs a unique nonce (a number used once);
//! reusing a nonce with the same key breaks the security. Use it to protect
//! files, database fields or messages between systems that share a key.

// Algorithm:
use aes_gcm::Aes256Gcm; // Use Aes128Gcm or Aes256Gcm, respectively.
// A 'nonce' is an arbitrary (often random) number
// that must be used just once
// in a cryptographic communication.
// Trait that provides `encrypt` and `decrypt`:
use aes_gcm::aead::{Aead as _, KeyInit as _};
// Cryptographic key and nonce types:
use aes_gcm::aead::{Generate as _, Key, Nonce};
use rand::rngs::SysRng;

/// Encrypts a message with a random key and decrypts it again.
fn main() -> anyhow::Result<()> {
    // Generate a random 256-bit key:
    let mut rng = SysRng;
    let key: Key<Aes256Gcm> =
        Key::<Aes256Gcm>::try_generate_from_rng(&mut rng)?;

    // Generate a fresh nonce. A production protocol must also coordinate
    // nonce allocation across every writer that shares this key.
    let nonce: Nonce<Aes256Gcm> =
        Nonce::<Aes256Gcm>::try_generate_from_rng(&mut rng)?;

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

        let mut rng = SysRng;
        let key: Key<Aes256Gcm> =
            Key::<Aes256Gcm>::try_generate_from_rng(&mut rng)?;
        let cipher = Aes256Gcm::new(&key);
        let nonce: Nonce<Aes256Gcm> =
            Nonce::<Aes256Gcm>::try_generate_from_rng(&mut rng)?;
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
