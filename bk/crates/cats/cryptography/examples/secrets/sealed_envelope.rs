// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Seals a secret under a passphrase: Argon2id derives a key, and
//! XChaCha20-Poly1305 encrypts the secret, authenticating a version label and
//! context as associated data.

use anyhow::Context as _;
use anyhow::Result;
use anyhow::anyhow;
use argon2::Algorithm;
use argon2::Argon2;
use argon2::Params;
use argon2::Version;
use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use chacha20poly1305::XChaCha20Poly1305;
use chacha20poly1305::XNonce;
use chacha20poly1305::aead::Aead as _;
use chacha20poly1305::aead::AeadCore as _;
use chacha20poly1305::aead::KeyInit as _;
use chacha20poly1305::aead::OsRng;
use chacha20poly1305::aead::Payload;
use chacha20poly1305::aead::rand_core::RngCore as _;
use serde::Deserialize;
use serde::Serialize;
use zeroize::Zeroizing;

/// The only envelope version this code reads and writes.
const VERSION: u32 = 1;
/// Salt length in bytes.
const SALT_LEN: usize = 16;
/// Nonce length in bytes for XChaCha20-Poly1305.
const NONCE_LEN: usize = 24;
/// Derived key length in bytes.
const KEY_LEN: usize = 32;
/// Argon2 memory cost in KiB (19 MiB: the OWASP minimum for `t = 2, p = 1`).
const MEMORY_KIB: u32 = 19 * 1024;
/// Argon2 iteration count.
const ITERATIONS: u32 = 2;
/// Argon2 lane count.
const PARALLELISM: u32 = 1;

/// A versioned, serializable envelope. It holds no secret: the passphrase is
/// never stored.
#[derive(Debug, Serialize, Deserialize)]
struct SealedEnvelope {
    /// Envelope format version, authenticated as part of the AAD.
    version: u32,
    /// Argon2 memory cost in KiB, stored so the cost can be raised later.
    memory_kib: u32,
    /// Argon2 iteration count.
    iterations: u32,
    /// Argon2 lane count.
    parallelism: u32,
    /// Base64 random salt.
    salt: String,
    /// Base64 random 24-byte nonce.
    nonce: String,
    /// Base64 ciphertext followed by the authentication tag.
    ciphertext: String,
}

/// Derives a 32-byte key from the passphrase with an explicit Argon2id cost.
fn derive_key(
    passphrase: &[u8],
    salt: &[u8],
    memory_kib: u32,
    iterations: u32,
    parallelism: u32,
) -> Result<Zeroizing<[u8; KEY_LEN]>> {
    let params =
        Params::new(memory_kib, iterations, parallelism, Some(KEY_LEN))
            .map_err(|e| anyhow!("invalid Argon2 parameters: {e}"))?;
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let mut key = Zeroizing::new([0_u8; KEY_LEN]);
    argon2
        .hash_password_into(passphrase, salt, key.as_mut())
        .map_err(|e| anyhow!("key derivation failed: {e}"))?;
    Ok(key)
}

/// Builds the associated data: it is authenticated but not encrypted, so
/// moving the envelope to another context makes decryption fail.
fn associated_data(version: u32, context: &[u8]) -> Vec<u8> {
    let mut aad = b"example-envelope".to_vec();
    aad.extend_from_slice(&version.to_be_bytes());
    aad.extend_from_slice(context);
    aad
}

impl SealedEnvelope {
    /// Encrypts `secret` under `passphrase`, bound to `context`.
    fn seal(secret: &[u8], passphrase: &[u8], context: &[u8]) -> Result<Self> {
        let mut salt = [0_u8; SALT_LEN];
        OsRng.fill_bytes(&mut salt);
        let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);
        let key =
            derive_key(passphrase, &salt, MEMORY_KIB, ITERATIONS, PARALLELISM)?;
        let cipher = XChaCha20Poly1305::new_from_slice(key.as_ref())
            .map_err(|e| anyhow!("invalid key: {e}"))?;
        let ciphertext = cipher
            .encrypt(
                &nonce,
                Payload {
                    msg: secret,
                    aad: &associated_data(VERSION, context),
                },
            )
            .map_err(|_e| anyhow!("encryption failed"))?;
        Ok(Self {
            version: VERSION,
            memory_kib: MEMORY_KIB,
            iterations: ITERATIONS,
            parallelism: PARALLELISM,
            salt: STANDARD.encode(salt),
            nonce: STANDARD.encode(nonce),
            ciphertext: STANDARD.encode(ciphertext),
        })
    }

    /// Decrypts the secret. Fails if the passphrase, context or any stored
    /// field is wrong or has been modified.
    fn open(
        &self,
        passphrase: &[u8],
        context: &[u8],
    ) -> Result<Zeroizing<Vec<u8>>> {
        if self.version != VERSION {
            return Err(anyhow!(
                "unsupported envelope version {}",
                self.version
            ));
        }
        let salt = STANDARD.decode(&self.salt).context("salt")?;
        let nonce = STANDARD.decode(&self.nonce).context("nonce")?;
        if nonce.len() != NONCE_LEN {
            return Err(anyhow!("nonce must be {NONCE_LEN} bytes"));
        }
        let ciphertext =
            STANDARD.decode(&self.ciphertext).context("ciphertext")?;
        let key = derive_key(
            passphrase,
            &salt,
            self.memory_kib,
            self.iterations,
            self.parallelism,
        )?;
        let cipher = XChaCha20Poly1305::new_from_slice(key.as_ref())
            .map_err(|e| anyhow!("invalid key: {e}"))?;
        let plaintext = cipher
            .decrypt(
                XNonce::from_slice(&nonce),
                Payload {
                    msg: &ciphertext,
                    aad: &associated_data(self.version, context),
                },
            )
            // Do not say which input was wrong.
            .map_err(|_e| anyhow!("decryption failed"))?;
        Ok(Zeroizing::new(plaintext))
    }
}

fn main() -> Result<()> {
    let envelope =
        SealedEnvelope::seal(b"sk-live-1234", b"correct horse", b"source-42")?;
    // The envelope is safe to store or ship as JSON.
    let json = serde_json::to_string_pretty(&envelope)?;
    println!("{json}");

    let restored: SealedEnvelope = serde_json::from_str(&json)?;
    let secret = restored.open(b"correct horse", b"source-42")?;
    assert_eq!(secret.as_slice(), b"sk-live-1234");
    Ok(())
}

// ANCHOR_END: example

pub fn run() {
    if let Err(e) = main() {
        eprintln!("{e:#}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip() -> Result<()> {
        let envelope = SealedEnvelope::seal(b"secret", b"pass", b"ctx")?;
        assert_eq!(envelope.open(b"pass", b"ctx")?.as_slice(), b"secret");
        Ok(())
    }

    #[test]
    fn wrong_passphrase_or_context_fails() -> Result<()> {
        let envelope = SealedEnvelope::seal(b"secret", b"pass", b"ctx")?;
        assert!(envelope.open(b"other", b"ctx").is_err());
        assert!(envelope.open(b"pass", b"other-ctx").is_err());
        Ok(())
    }

    #[test]
    fn tampered_ciphertext_fails() -> Result<()> {
        let mut envelope = SealedEnvelope::seal(b"secret", b"pass", b"ctx")?;
        let mut bytes = STANDARD.decode(&envelope.ciphertext)?;
        bytes[0] ^= 1;
        envelope.ciphertext = STANDARD.encode(bytes);
        assert!(envelope.open(b"pass", b"ctx").is_err());
        Ok(())
    }

    #[test]
    fn tampered_version_fails() -> Result<()> {
        let mut envelope = SealedEnvelope::seal(b"secret", b"pass", b"ctx")?;
        envelope.version = 2;
        assert!(envelope.open(b"pass", b"ctx").is_err());
        Ok(())
    }

    /// Runs the sealed envelope example through its entry point.
    #[test]
    fn test_sealed_envelope_example_entry_point() {
        run();
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
