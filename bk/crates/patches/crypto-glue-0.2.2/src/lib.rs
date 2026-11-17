//! RustCrypto facade for composing cryptographic primitives and X.509 helpers.
//!
//! TODO Reviewed ??-??-2026.
//!
//! The facade keeps cryptographic operations on compatible RustCrypto APIs.
#![deny(warnings)]
#![allow(dead_code)]
#![warn(unused_extern_crates)]
// Enable some groups of clippy lints.
#![deny(clippy::suspicious)]
#![deny(clippy::perf)]
// Specific lints to enforce.
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::await_holding_lock)]
#![deny(clippy::needless_pass_by_value)]
#![deny(clippy::trivially_copy_pass_by_ref)]
#![deny(clippy::disallowed_types)]
#![deny(clippy::manual_let_else)]
#![allow(clippy::unreachable)]

pub use argon2;
pub use cipher::block_padding;
pub use der;
pub use hex;
pub use pbkdf2;
pub use rand;
pub use spki;
pub use zeroize;

/// Provides prelude behavior for the src component.
pub mod prelude {}

/// Provides test ca behavior for the src component.
#[cfg(test)]
mod test_ca;

/// Provides traits behavior for the src component.
pub mod traits {
    pub use aes_gcm::aead::AeadInOut;
    pub use crypto_common::Generate;
    pub use crypto_common::KeyInit;
    pub use crypto_common::OutputSizeUser;
    pub use der::Decode as DecodeDer;
    pub use der::DecodePem;
    pub use der::Encode as EncodeDer;
    pub use der::EncodePem;
    pub use der::pem::LineEnding as LineEndingPem;
    pub use der::referenced::OwnedToRef;
    pub use digest::FixedOutput;
    pub use elliptic_curve::sec1::FromSec1Point;
    pub use elliptic_curve::sec1::ToSec1Point;
    pub use hmac::Hmac;
    pub use hmac::Mac;
    pub use pkcs8::DecodePrivateKey as Pkcs8DecodePrivateKey;
    pub use pkcs8::EncodePrivateKey as Pkcs8EncodePrivateKey;
    pub use rsa::pkcs1::DecodeRsaPrivateKey as Pkcs1DecodeRsaPrivateKey;
    pub use rsa::pkcs1::EncodeRsaPrivateKey as Pkcs1EncodeRsaPrivateKey;
    pub use rsa::signature::DigestSigner;
    pub use rsa::signature::DigestVerifier;
    pub use rsa::signature::Keypair;
    pub use rsa::signature::RandomizedSigner;
    pub use rsa::signature::SignatureEncoding;
    pub use rsa::signature::Signer;
    pub use rsa::signature::Verifier;
    pub use rsa::traits::PublicKeyParts;
    pub use sha2::Digest;
    pub use spki::DecodePublicKey as SpkiDecodePublicKey;
    pub use spki::DynSignatureAlgorithmIdentifier;
    pub use spki::EncodePublicKey as SpkiEncodePublicKey;
    pub use zeroize::Zeroizing;
    /// Provides hazmat behavior for the src component.
    pub mod hazmat {

        //! This is a “Hazardous Materials” module. You should ONLY use it if you’re 100% absolutely
        //! sure that you know what you’re doing because this module is full of land mines, dragons,
        //! and dinosaurs with laser guns.

        pub use rsa::signature::hazmat::PrehashVerifier;
    }
    pub use x509_cert::ext::ToExtension;
}

/// Provides x509 behavior for the src component.
pub mod x509;

/// Provides md5 behavior for the src component.
pub mod md5 {
    pub use md5::*;
}

/// Provides sha1 behavior for the src component.
pub mod sha1 {
    use hybrid_array::Array;
    use hybrid_array::sizes::U20;
    pub use sha1::Sha1;

    /// Alias for `Array<u8, U20>` used to represent sha1 output.
    pub type Sha1Output = Array<u8, U20>;
}

/// Provides s256 behavior for the src component.
pub mod s256 {
    use hybrid_array::Array;
    use hybrid_array::sizes::U32;
    pub use sha2::Sha256;

    /// Alias for `Array<u8, U32>` used to represent sha256 output.
    pub type Sha256Output = Array<u8, U32>;
}

/// Provides s384 behavior for the src component.
pub mod s384 {
    use hybrid_array::Array;
    use hybrid_array::sizes::U48;
    pub use sha2::Sha384;

    /// Alias for `Array<u8, U48>` used to represent sha384 output.
    pub type Sha384Output = Array<u8, U48>;
}

/// Provides s512 behavior for the src component.
pub mod s512 {
    use hybrid_array::Array;
    use hybrid_array::sizes::U64;
    pub use sha2::Sha512;

    /// Alias for `Array<u8, U64>` used to represent sha512 output.
    pub type Sha512Output = Array<u8, U64>;
}

/// Provides hkdf s256 behavior for the src component.
pub mod hkdf_s256 {
    use hkdf::Hkdf;
    use sha2::Sha256;

    /// Alias for `Hkdf<Sha256>` used to represent hkdf sha256.
    pub type HkdfSha256 = Hkdf<Sha256>;
}

/// Provides hmac s1 behavior for the src component.
pub mod hmac_s1 {
    use crypto_common::Key;
    use crypto_common::Output;
    use hmac::Hmac;
    use hmac::Mac;
    use sha1::Sha1;
    use sha1::digest::CtOutput;
    use zeroize::Zeroizing;

    /// Alias for `Hmac<Sha1>` used to represent hmac sha1.
    pub type HmacSha1 = Hmac<Sha1>;

    /// Alias for `Zeroizing<Key<Hmac<Sha1>>>` used to represent hmac sha1 key.
    pub type HmacSha1Key = Zeroizing<Key<Hmac<Sha1>>>;

    /// Alias for `CtOutput<HmacSha1>` used to represent hmac sha1 output.
    pub type HmacSha1Output = CtOutput<HmacSha1>;

    /// Alias for `Output<HmacSha1>` used to represent hmac sha1 bytes.
    pub type HmacSha1Bytes = Output<HmacSha1>;

    /// Creates a cryptographic key from the supplied key material.
    pub fn new_key() -> HmacSha1Key {
        use crypto_common::Generate;
        Key::<HmacSha1>::generate().into()
    }

    /// Performs the one-shot operation using the supplied key and input.
    pub fn oneshot(key: &HmacSha1Key, data: &[u8]) -> HmacSha1Output {
        use crypto_common::KeyInit;

        let mut hmac = HmacSha1::new(key);
        hmac.update(data);
        hmac.finalize()
    }

    /// Constructs a key from vec bytes.
    #[allow(clippy::needless_pass_by_value)]
    pub fn key_from_vec(bytes: Vec<u8>) -> Option<HmacSha1Key> {
        key_from_slice(&bytes)
    }

    /// Constructs a key from slice bytes.
    pub fn key_from_slice(bytes: &[u8]) -> Option<HmacSha1Key> {
        use crypto_common::KeySizeUser;
        // Key too short - too long.
        if bytes.len() < 16 || bytes.len() > Hmac::<Sha1>::key_size() {
            None
        } else {
            let mut key = Key::<Hmac<Sha1>>::default();
            let key_ref = &mut key.as_mut_slice()[..bytes.len()];
            key_ref.copy_from_slice(bytes);
            Some(key.into())
        }
    }

    /// Constructs a key from bytes bytes.
    pub fn key_from_bytes(bytes: [u8; 64]) -> HmacSha1Key {
        Key::<Hmac<Sha1>>::from(bytes).into()
    }

    /// Returns the number of bytes required to represent a key.
    pub fn key_size() -> usize {
        use crypto_common::KeySizeUser;
        Hmac::<Sha1>::key_size()
    }
}

/// Provides hmac s256 behavior for the src component.
pub mod hmac_s256 {
    use crypto_common::Key;
    use crypto_common::Output;
    use hmac::Hmac;
    use hmac::Mac;
    use sha2::Sha256;
    use sha2::digest::CtOutput;
    use zeroize::Zeroizing;

    /// Alias for `Hmac<Sha256>` used to represent hmac sha256.
    pub type HmacSha256 = Hmac<Sha256>;

    /// Alias for `Zeroizing<Key<Hmac<Sha256>>>` used to represent hmac sha256 key.
    pub type HmacSha256Key = Zeroizing<Key<Hmac<Sha256>>>;

    /// Alias for `CtOutput<HmacSha256>` used to represent hmac sha256 output.
    pub type HmacSha256Output = CtOutput<HmacSha256>;

    /// Alias for `Output<HmacSha256>` used to represent hmac sha256 bytes.
    pub type HmacSha256Bytes = Output<HmacSha256>;

    /// Creates a cryptographic key from the supplied key material.
    pub fn new_key() -> HmacSha256Key {
        use crypto_common::Generate;
        Key::<HmacSha256>::generate().into()
    }

    /// Performs the one-shot operation using the supplied key and input.
    pub fn oneshot(key: &HmacSha256Key, data: &[u8]) -> HmacSha256Output {
        use crypto_common::KeyInit;

        let mut hmac = HmacSha256::new(key);
        hmac.update(data);
        hmac.finalize()
    }

    /// Constructs a key from vec bytes.
    #[allow(clippy::needless_pass_by_value)]
    pub fn key_from_vec(bytes: Vec<u8>) -> Option<HmacSha256Key> {
        key_from_slice(&bytes)
    }

    /// Constructs a key from slice bytes.
    pub fn key_from_slice(bytes: &[u8]) -> Option<HmacSha256Key> {
        use crypto_common::KeySizeUser;
        // Key too short - too long.
        if bytes.len() < 16 || bytes.len() > Hmac::<Sha256>::key_size() {
            None
        } else {
            let mut key = Key::<Hmac<Sha256>>::default();
            let key_ref = &mut key.as_mut_slice()[..bytes.len()];
            key_ref.copy_from_slice(bytes);
            Some(key.into())
        }
    }

    /// Constructs a key from bytes bytes.
    pub fn key_from_bytes(bytes: [u8; 64]) -> HmacSha256Key {
        Key::<Hmac<Sha256>>::from(bytes).into()
    }

    /// Returns the number of bytes required to represent a key.
    pub fn key_size() -> usize {
        use crypto_common::KeySizeUser;
        Hmac::<Sha256>::key_size()
    }
}

/// Provides hmac s512 behavior for the src component.
pub mod hmac_s512 {
    use crypto_common::Key;
    use crypto_common::Output;
    use hmac::Hmac;
    pub use hmac::Mac;
    use sha2::Sha512;
    use sha2::digest::CtOutput;
    use zeroize::Zeroizing;

    /// Alias for `Hmac<Sha512>` used to represent hmac sha512.
    pub type HmacSha512 = Hmac<Sha512>;

    /// Alias for `Zeroizing<Key<Hmac<Sha512>>>` used to represent hmac sha512 key.
    pub type HmacSha512Key = Zeroizing<Key<Hmac<Sha512>>>;

    /// Alias for `CtOutput<HmacSha512>` used to represent hmac sha512 output.
    pub type HmacSha512Output = CtOutput<HmacSha512>;

    /// Alias for `Output<HmacSha512>` used to represent hmac sha512 bytes.
    pub type HmacSha512Bytes = Output<HmacSha512>;

    /// Handles the new hmac sha512 key behavior for this component.
    pub fn new_hmac_sha512_key() -> HmacSha512Key {
        use crypto_common::Generate;
        Key::<HmacSha512>::generate().into()
    }

    /// Performs the one-shot operation using the supplied key and input.
    pub fn oneshot(key: &HmacSha512Key, data: &[u8]) -> HmacSha512Output {
        use crypto_common::KeyInit;

        let mut hmac = HmacSha512::new(key);
        hmac.update(data);
        hmac.finalize()
    }

    /// Constructs a key from slice bytes.
    pub fn key_from_slice(bytes: &[u8]) -> Option<HmacSha512Key> {
        use crypto_common::KeySizeUser;
        // Key too short - too long.
        if bytes.len() < 16 || bytes.len() > Hmac::<Sha512>::key_size() {
            None
        } else {
            let mut key = Key::<Hmac<Sha512>>::default();
            let key_ref = &mut key.as_mut_slice()[..bytes.len()];
            key_ref.copy_from_slice(bytes);
            Some(key.into())
        }
    }

    /// Returns the number of bytes required to represent a key.
    pub fn key_size() -> usize {
        use crypto_common::KeySizeUser;
        Hmac::<Sha512>::key_size()
    }
}

/// Provides aes128 behavior for the src component.
pub mod aes128 {
    use aes;
    use crypto_common::Key;
    use zeroize::Zeroizing;

    /// Alias for `Zeroizing<Key<aes::Aes128>>` used to represent aes128 key.
    pub type Aes128Key = Zeroizing<Key<aes::Aes128>>;

    /// Returns the number of bytes required to represent a key.
    pub fn key_size() -> usize {
        use crypto_common::KeySizeUser;
        aes::Aes128::key_size()
    }

    /// Constructs a key from slice bytes.
    pub fn key_from_slice(bytes: &[u8]) -> Option<Aes128Key> {
        Key::<aes::Aes128>::try_from(bytes)
            .ok()
            .map(|key| key.into())
    }

    /// Constructs a key from bytes bytes.
    pub fn key_from_bytes(bytes: [u8; 16]) -> Aes128Key {
        Key::<aes::Aes128>::from(bytes).into()
    }

    /// Creates a cryptographic key from the supplied key material.
    pub fn new_key() -> Aes128Key {
        use crypto_common::Generate;
        Key::<aes::Aes128>::generate().into()
    }
}

/// Provides aes128gcm behavior for the src component.
pub mod aes128gcm {
    use aes::cipher::consts::U12;
    use aes::cipher::consts::U16;
    pub use aes_gcm::aead::Aead;
    pub use aes_gcm::aead::AeadInOut;
    pub use aes_gcm::aead::Payload;
    pub use crypto_common::KeyInit;

    pub use crate::aes128::Aes128Key;

    /// Alias for `aes_gcm::Aes128Gcm` used to represent aes128 gcm.
    pub type Aes128Gcm = aes_gcm::Aes128Gcm;

    /// Alias for `aes_gcm::Nonce<U12>` used to represent aes128 gcm nonce.
    pub type Aes128GcmNonce = aes_gcm::Nonce<U12>;
    /// Alias for `aes_gcm::Tag<U16>` used to represent aes128 gcm tag.
    pub type Aes128GcmTag = aes_gcm::Tag<U16>;

    /// Handles the new nonce behavior for this component.
    pub fn new_nonce() -> Aes128GcmNonce {
        use crypto_common::Generate;
        Aes128GcmNonce::generate()
    }
}

/// Provides aes128kw behavior for the src component.
pub mod aes128kw {
    pub use crypto_common::KeyInit;
    use hybrid_array::Array;
    use hybrid_array::sizes::U24;

    /// Alias for `aes_kw::KwAes128` used to represent aes128 kw.
    pub type Aes128Kw = aes_kw::KwAes128;

    /// Alias for `Array<u8, U24>` used to represent aes128 kw wrapped.
    pub type Aes128KwWrapped = Array<u8, U24>;
}

/// Provides aes256 behavior for the src component.
pub mod aes256 {
    use aes;
    pub use aes::Aes256;
    use aes::cipher::Array;
    pub use aes::cipher::BlockCipherDecrypt;
    pub use aes::cipher::BlockCipherEncrypt;
    use crypto_common::Key;
    use zeroize::Zeroizing;

    /// Alias for `Zeroizing<Key<aes::Aes256>>` used to represent aes256 key.
    pub type Aes256Key = Zeroizing<Key<aes::Aes256>>;
    /// Alias for `<aes::Aes256 as aes::cipher::BlockSizeUser>::BlockSize` used to represent aes256
    /// block size.
    pub type Aes256BlockSize = <aes::Aes256 as aes::cipher::BlockSizeUser>::BlockSize;
    /// Alias for `Array<u8, <aes::Aes256 as aes::cipher::BlockSizeUser>::BlockSize>` used to
    /// represent aes256 block.
    pub type Aes256Block = Array<u8, <aes::Aes256 as aes::cipher::BlockSizeUser>::BlockSize>;

    /// Returns the number of bytes required to represent a key.
    pub fn key_size() -> usize {
        use crypto_common::KeySizeUser;
        aes::Aes256::key_size()
    }

    /// Constructs a key from slice bytes.
    pub fn key_from_slice(bytes: &[u8]) -> Option<Aes256Key> {
        Key::<aes::Aes256>::try_from(bytes)
            .ok()
            .map(|key| key.into())
    }

    /// Constructs a key from bytes bytes.
    pub fn key_from_bytes(bytes: [u8; 32]) -> Aes256Key {
        Key::<aes::Aes256>::from(bytes).into()
    }

    /// Creates a cryptographic key from the supplied key material.
    pub fn new_key() -> Aes256Key {
        use crypto_common::Generate;
        Key::<aes::Aes256>::generate().into()
    }
}

/// Provides aes256gcm behavior for the src component.
pub mod aes256gcm {
    use aes::Aes256;
    use aes::cipher::consts::U12;
    use aes::cipher::consts::U16;
    use aes_gcm::AesGcm;
    pub use aes_gcm::aead::Aead;
    pub use aes_gcm::aead::AeadInOut;
    pub use aes_gcm::aead::Payload;
    pub use crypto_common::KeyInit;

    pub use crate::aes256::Aes256Key;

    // Same as  AesGcm<Aes256, U12, U16>;
    /// Alias for `aes_gcm::Aes256Gcm` used to represent aes256 gcm.
    pub type Aes256Gcm = aes_gcm::Aes256Gcm;

    /// Alias for `AesGcm<Aes256, U16, U16>` used to represent aes256 gcm n16.
    pub type Aes256GcmN16 = AesGcm<Aes256, U16, U16>;
    /// Alias for `aes_gcm::Nonce<U16>` used to represent aes256 gcm nonce16.
    pub type Aes256GcmNonce16 = aes_gcm::Nonce<U16>;

    /// Alias for `aes_gcm::Nonce<U12>` used to represent aes256 gcm nonce.
    pub type Aes256GcmNonce = aes_gcm::Nonce<U12>;
    /// Alias for `aes_gcm::Tag<U16>` used to represent aes256 gcm tag.
    pub type Aes256GcmTag = aes_gcm::Tag<U16>;

    /// Handles the new nonce behavior for this component.
    pub fn new_nonce() -> Aes256GcmNonce {
        use crypto_common::Generate;
        Aes256GcmNonce::generate()
    }
}

/// Provides aes256cts behavior for the src component.
pub mod aes256cts {
    use aes::cipher::Array;
    pub use aes::cipher::BlockModeDecrypt;
    pub use aes::cipher::BlockModeEncrypt;
    pub use aes::cipher::InnerIvInit;
    pub use aes::cipher::KeyIvInit;
    use aes::cipher::consts::U16;
    pub use cts::Decrypt as CtsDecrypt;
    pub use cts::Encrypt as CtsEncrypt;

    pub use crate::aes256::Aes256Key;

    /// Alias for `cts::CbcCs3<aes::Aes256>` used to represent aes256 cts enc.
    pub type Aes256CtsEnc = cts::CbcCs3<aes::Aes256>;
    /// Alias for `cts::CbcCs3<aes::Aes256>` used to represent aes256 cts dec.
    pub type Aes256CtsDec = cts::CbcCs3<aes::Aes256>;

    /// Alias for `Array<u8, U16>` used to represent aes256 cts iv.
    pub type Aes256CtsIv = Array<u8, U16>;

    /// Handles the new iv behavior for this component.
    pub fn new_iv() -> Aes256CtsIv {
        use crypto_common::Generate;
        Aes256CtsIv::generate()
    }
}

/// Provides aes256cbc behavior for the src component.
pub mod aes256cbc {
    use aes::cipher::Array;
    pub use aes::cipher::BlockModeDecrypt;
    pub use aes::cipher::BlockModeEncrypt;
    pub use aes::cipher::KeyIvInit;
    pub use aes::cipher::block_padding;
    use aes::cipher::consts::U16;

    pub use crate::aes256::Aes256Key;
    use crate::hmac_s256::HmacSha256;
    use crate::hmac_s256::HmacSha256Output;

    /// Alias for `cbc::Encryptor<aes::Aes256>` used to represent aes256 cbc enc.
    pub type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;
    /// Alias for `cbc::Decryptor<aes::Aes256>` used to represent aes256 cbc dec.
    pub type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

    /// Alias for `Array<u8, U16>` used to represent aes256 cbc iv.
    pub type Aes256CbcIv = Array<u8, U16>;

    /// Handles the new iv behavior for this component.
    pub fn new_iv() -> Aes256CbcIv {
        use crypto_common::Generate;
        Aes256CbcIv::generate()
    }

    /// Handles the enc behavior for this component.
    pub fn enc<P>(
        key: &Aes256Key,
        data: &[u8],
    ) -> Result<(HmacSha256Output, Aes256CbcIv, Vec<u8>), crypto_common::InvalidLength>
    where
        P: block_padding::Padding,
    {
        use cipher::BlockModeEncrypt;
        use cipher::KeyInit;
        use hmac::Mac;

        let iv = new_iv();
        let enc = Aes256CbcEnc::new(key, &iv);

        let ciphertext = enc.encrypt_padded_vec::<P>(data);

        let mut hmac = HmacSha256::new_from_slice(key.as_slice())?;
        hmac.update(&ciphertext);
        let mac = hmac.finalize();

        Ok((mac, iv, ciphertext))
    }

    /// Handles the dec behavior for this component.
    pub fn dec<P>(
        key: &Aes256Key,
        mac: &HmacSha256Output,
        iv: &Aes256CbcIv,
        ciphertext: &[u8],
    ) -> Option<Vec<u8>>
    where
        P: block_padding::Padding,
    {
        use cipher::BlockModeDecrypt;
        use cipher::KeyInit;
        use hmac::Mac;

        let mut hmac = HmacSha256::new_from_slice(key.as_slice()).ok()?;
        hmac.update(ciphertext);
        let check_mac = hmac.finalize();

        if check_mac != *mac {
            return None;
        }

        let dec = Aes256CbcDec::new(key, iv);

        let plaintext = dec.decrypt_padded_vec::<P>(ciphertext).ok()?;

        Some(plaintext)
    }
}

/// Provides aes256kw behavior for the src component.
pub mod aes256kw {
    pub use crypto_common::KeyInit;
    use hybrid_array::Array;
    use hybrid_array::sizes::U40;

    /// Alias for `aes_kw::KwAes256` used to represent aes256 kw.
    pub type Aes256Kw = aes_kw::KwAes256;

    /// Alias for `Array<u8, U40>` used to represent aes256 kw wrapped.
    pub type Aes256KwWrapped = Array<u8, U40>;
}

/// Provides rsa behavior for the src component.
pub mod rsa {
    pub use rand;
    pub use rsa::BoxedUint as BigUint;
    pub use rsa::Oaep;
    use rsa::PrivateKey as RsaPrivateKey;
    use rsa::PublicKey as RsaPublicKey;
    pub use rsa::pkcs1v15;
    use rsa::pkcs1v15::Signature;
    use rsa::pkcs1v15::SigningKey;
    use rsa::pkcs1v15::VerifyingKey;
    pub use sha2::Sha256;
    pub use sha2::Sha384;

    /// Sets the min bits bound or default used by this code.
    pub const MIN_BITS: usize = 2048;

    /// Alias for `RsaPrivateKey` used to represent rs256 private key.
    pub type RS256PrivateKey = RsaPrivateKey;
    /// Alias for `RsaPublicKey` used to represent rs256 public key.
    pub type RS256PublicKey = RsaPublicKey;
    /// Alias for `Signature` used to represent rs256 signature.
    pub type RS256Signature = Signature;
    /// Alias for `Sha256` used to represent rs256 digest.
    pub type RS256Digest = Sha256;
    /// Alias for `VerifyingKey<Sha256>` used to represent rs256 verifying key.
    pub type RS256VerifyingKey = VerifyingKey<Sha256>;
    /// Alias for `SigningKey<Sha256>` used to represent rs256 signing key.
    pub type RS256SigningKey = SigningKey<Sha256>;

    /// Alias for `Sha384` used to represent rs384 digest.
    pub type RS384Digest = Sha384;
    /// Alias for `VerifyingKey<Sha384>` used to represent rs384 verifying key.
    pub type RS384VerifyingKey = VerifyingKey<Sha384>;
    /// Alias for `SigningKey<Sha384>` used to represent rs384 signing key.
    pub type RS384SigningKey = SigningKey<Sha384>;

    /// Creates a cryptographic key from the supplied key material.
    pub fn new_key(bits: usize) -> rsa::errors::Result<RsaPrivateKey> {
        let bits = std::cmp::max(bits, MIN_BITS);
        let mut rng = rand::rng();
        RsaPrivateKey::new(&mut rng, bits)
    }

    /// Handles the oaep sha256 encrypt behavior for this component.
    pub fn oaep_sha256_encrypt(
        public_key: &RsaPublicKey,
        data: &[u8],
    ) -> rsa::errors::Result<Vec<u8>> {
        let mut rng = rand::rng();
        let padding = Oaep::<Sha256>::new();
        public_key.encrypt(&mut rng, padding, data)
    }

    /// Handles the oaep sha256 decrypt behavior for this component.
    pub fn oaep_sha256_decrypt(
        private_key: &RsaPrivateKey,
        ciphertext: &[u8],
    ) -> rsa::errors::Result<Vec<u8>> {
        let padding = Oaep::<Sha256>::new();
        private_key.decrypt(padding, ciphertext)
    }
}

/// Provides ec behavior for the src component.
pub mod ec {
    pub use sec1::EcPrivateKey;
}

/// Provides ecdh behavior for the src component.
pub mod ecdh {
    pub use elliptic_curve::ecdh::diffie_hellman;
}

/// Provides ecdh p256 behavior for the src component.
pub mod ecdh_p256 {
    use elliptic_curve::FieldBytes;
    use elliptic_curve::PublicKey;
    use elliptic_curve::ecdh::EphemeralSecret;
    use elliptic_curve::ecdh::SharedSecret;
    use elliptic_curve::sec1::Sec1Point;
    use hkdf::Hkdf;
    use p256::NistP256;
    use sha2::Sha256;

    /// Alias for `EphemeralSecret<NistP256>` used to represent ecdh p256 ephemeral secret.
    pub type EcdhP256EphemeralSecret = EphemeralSecret<NistP256>;
    /// Alias for `SharedSecret<NistP256>` used to represent ecdh p256 shared secret.
    pub type EcdhP256SharedSecret = SharedSecret<NistP256>;
    /// Alias for `PublicKey<NistP256>` used to represent ecdh p256 public key.
    pub type EcdhP256PublicKey = PublicKey<NistP256>;
    /// Alias for `Sec1Point<NistP256>` used to represent ecdh p256 public sec1 point.
    pub type EcdhP256PublicSec1Point = Sec1Point<NistP256>;
    /// Alias for `FieldBytes<NistP256>` used to represent ecdh p256 field bytes.
    pub type EcdhP256FieldBytes = FieldBytes<NistP256>;

    /// Alias for `Hkdf<Sha256>` used to represent ecdh p256 hkdf.
    pub type EcdhP256Hkdf = Hkdf<Sha256>;

    /// Alias for `Sha256` used to represent ecdh p256 digest.
    pub type EcdhP256Digest = Sha256;

    /// Handles the new secret behavior for this component.
    pub fn new_secret() -> EcdhP256EphemeralSecret {
        use crypto_common::Generate;
        EcdhP256EphemeralSecret::generate()
    }
}

/// Provides ecdsa p256 behavior for the src component.
pub mod ecdsa_p256 {
    use ecdsa::DigestAlgorithm;
    use ecdsa::Signature;
    use ecdsa::SignatureBytes;
    use ecdsa::SigningKey;
    use ecdsa::VerifyingKey;
    use elliptic_curve::FieldBytes;
    use elliptic_curve::PublicKey;
    use elliptic_curve::SecretKey;
    use elliptic_curve::point::AffinePoint;
    use elliptic_curve::scalar::NonZeroScalar;
    use elliptic_curve::sec1::FromSec1Point;
    use elliptic_curve::sec1::Sec1Point;
    use hybrid_array::Array;
    use hybrid_array::sizes::U32;
    use p256::NistP256;
    use p256::ecdsa::DerSignature;

    /// Alias for `<NistP256 as DigestAlgorithm>::Digest` used to represent ecdsa p256 digest.
    pub type EcdsaP256Digest = <NistP256 as DigestAlgorithm>::Digest;

    /// Alias for `SecretKey<NistP256>` used to represent ecdsa p256 private key.
    pub type EcdsaP256PrivateKey = SecretKey<NistP256>;
    /// Alias for `NonZeroScalar<NistP256>` used to represent ecdsa p256 non zero scalar.
    pub type EcdsaP256NonZeroScalar = NonZeroScalar<NistP256>;

    /// Alias for `FieldBytes<NistP256>` used to represent ecdsa p256 field bytes.
    pub type EcdsaP256FieldBytes = FieldBytes<NistP256>;
    /// Alias for `AffinePoint<NistP256>` used to represent ecdsa p256 affine point.
    pub type EcdsaP256AffinePoint = AffinePoint<NistP256>;

    /// Alias for `PublicKey<NistP256>` used to represent ecdsa p256 public key.
    pub type EcdsaP256PublicKey = PublicKey<NistP256>;

    /// Alias for `Array<u8, U32>` used to represent ecdsa p256 public coordinate.
    pub type EcdsaP256PublicCoordinate = Array<u8, U32>;
    /// Alias for `Sec1Point<NistP256>` used to represent ecdsa p256 public sec1 point.
    pub type EcdsaP256PublicSec1Point = Sec1Point<NistP256>;

    /// Alias for `SigningKey<NistP256>` used to represent ecdsa p256 signing key.
    pub type EcdsaP256SigningKey = SigningKey<NistP256>;
    /// Alias for `VerifyingKey<NistP256>` used to represent ecdsa p256 verifying key.
    pub type EcdsaP256VerifyingKey = VerifyingKey<NistP256>;

    /// Alias for `Signature<NistP256>` used to represent ecdsa p256 signature.
    pub type EcdsaP256Signature = Signature<NistP256>;
    /// Alias for `DerSignature` used to represent ecdsa p256 der signature.
    pub type EcdsaP256DerSignature = DerSignature;
    /// Alias for `SignatureBytes<NistP256>` used to represent ecdsa p256 signature bytes.
    pub type EcdsaP256SignatureBytes = SignatureBytes<NistP256>;

    /// Creates a cryptographic key from the supplied key material.
    pub fn new_key() -> EcdsaP256PrivateKey {
        use crypto_common::Generate;

        EcdsaP256PrivateKey::generate()
    }

    /// Constructs this value from a coords raw.
    pub fn from_coords_raw(x: &[u8], y: &[u8]) -> Option<EcdsaP256PublicKey> {
        let mut field_x = EcdsaP256FieldBytes::default();
        if x.len() != field_x.len() {
            return None;
        }

        let mut field_y = EcdsaP256FieldBytes::default();
        if y.len() != field_y.len() {
            return None;
        }

        field_x.copy_from_slice(x);
        field_y.copy_from_slice(y);

        let ep = EcdsaP256PublicSec1Point::from_affine_coordinates(&field_x, &field_y, false);

        EcdsaP256PublicKey::from_sec1_point(&ep).into_option()
    }
}

/// Provides ecdsa p384 behavior for the src component.
pub mod ecdsa_p384 {
    use ecdsa::DigestAlgorithm;
    use ecdsa::Signature;
    use ecdsa::SignatureBytes;
    use ecdsa::SigningKey;
    use ecdsa::VerifyingKey;
    use elliptic_curve::FieldBytes;
    use elliptic_curve::PublicKey;
    use elliptic_curve::SecretKey;
    use elliptic_curve::point::AffinePoint;
    use elliptic_curve::sec1::FromSec1Point;
    use elliptic_curve::sec1::Sec1Point;
    use p384::NistP384;
    use p384::ecdsa::DerSignature;
    // use sha2::digest::consts::U32;

    /// Alias for `<NistP384 as DigestAlgorithm>::Digest` used to represent ecdsa p384 digest.
    pub type EcdsaP384Digest = <NistP384 as DigestAlgorithm>::Digest;

    /// Alias for `SecretKey<NistP384>` used to represent ecdsa p384 private key.
    pub type EcdsaP384PrivateKey = SecretKey<NistP384>;

    /// Alias for `FieldBytes<NistP384>` used to represent ecdsa p384 field bytes.
    pub type EcdsaP384FieldBytes = FieldBytes<NistP384>;
    /// Alias for `AffinePoint<NistP384>` used to represent ecdsa p384 affine point.
    pub type EcdsaP384AffinePoint = AffinePoint<NistP384>;

    /// Alias for `PublicKey<NistP384>` used to represent ecdsa p384 public key.
    pub type EcdsaP384PublicKey = PublicKey<NistP384>;

    // pub type EcdsaP384PublicCoordinate = GenericArray<u8, U32>;
    /// Alias for `Sec1Point<NistP384>` used to represent ecdsa p384 public sec1 point.
    pub type EcdsaP384PublicSec1Point = Sec1Point<NistP384>;

    /// Alias for `SigningKey<NistP384>` used to represent ecdsa p384 signing key.
    pub type EcdsaP384SigningKey = SigningKey<NistP384>;
    /// Alias for `VerifyingKey<NistP384>` used to represent ecdsa p384 verifying key.
    pub type EcdsaP384VerifyingKey = VerifyingKey<NistP384>;

    /// Alias for `Signature<NistP384>` used to represent ecdsa p384 signature.
    pub type EcdsaP384Signature = Signature<NistP384>;
    /// Alias for `DerSignature` used to represent ecdsa p384 der signature.
    pub type EcdsaP384DerSignature = DerSignature;
    /// Alias for `SignatureBytes<NistP384>` used to represent ecdsa p384 signature bytes.
    pub type EcdsaP384SignatureBytes = SignatureBytes<NistP384>;

    /// Creates a cryptographic key from the supplied key material.
    pub fn new_key() -> EcdsaP384PrivateKey {
        use crypto_common::Generate;
        EcdsaP384PrivateKey::generate()
    }

    /// Constructs this value from a coords raw.
    pub fn from_coords_raw(x: &[u8], y: &[u8]) -> Option<EcdsaP384PublicKey> {
        let mut field_x = EcdsaP384FieldBytes::default();
        if x.len() != field_x.len() {
            return None;
        }

        let mut field_y = EcdsaP384FieldBytes::default();
        if y.len() != field_y.len() {
            return None;
        }

        field_x.copy_from_slice(x);
        field_y.copy_from_slice(y);

        let ep = EcdsaP384PublicSec1Point::from_affine_coordinates(&field_x, &field_y, false);

        EcdsaP384PublicKey::from_sec1_point(&ep).into_option()
    }
}

/// Provides ecdsa p521 behavior for the src component.
pub mod ecdsa_p521 {
    use ecdsa::DigestAlgorithm;
    use ecdsa::Signature;
    use ecdsa::SignatureBytes;
    use ecdsa::SigningKey;
    use ecdsa::VerifyingKey;
    use elliptic_curve::FieldBytes;
    use elliptic_curve::PublicKey;
    use elliptic_curve::SecretKey;
    use elliptic_curve::point::AffinePoint;
    use elliptic_curve::sec1::FromSec1Point;
    use elliptic_curve::sec1::Sec1Point;
    use p521::NistP521;
    use p521::ecdsa::DerSignature;

    /// Alias for `<NistP521 as DigestAlgorithm>::Digest` used to represent ecdsa p521 digest.
    pub type EcdsaP521Digest = <NistP521 as DigestAlgorithm>::Digest;

    /// Alias for `SecretKey<NistP521>` used to represent ecdsa p521 private key.
    pub type EcdsaP521PrivateKey = SecretKey<NistP521>;

    /// Alias for `FieldBytes<NistP521>` used to represent ecdsa p521 field bytes.
    pub type EcdsaP521FieldBytes = FieldBytes<NistP521>;
    /// Alias for `AffinePoint<NistP521>` used to represent ecdsa p521 affine point.
    pub type EcdsaP521AffinePoint = AffinePoint<NistP521>;

    /// Alias for `PublicKey<NistP521>` used to represent ecdsa p521 public key.
    pub type EcdsaP521PublicKey = PublicKey<NistP521>;

    // pub type EcdsaP521PublicCoordinate = GenericArray<u8, U32>;
    /// Alias for `Sec1Point<NistP521>` used to represent ecdsa p521 public sec1 point.
    pub type EcdsaP521PublicSec1Point = Sec1Point<NistP521>;

    /// Alias for `SigningKey<NistP521>` used to represent ecdsa p521 signing key.
    pub type EcdsaP521SigningKey = SigningKey<NistP521>;
    /// Alias for `VerifyingKey<NistP521>` used to represent ecdsa p521 verifying key.
    pub type EcdsaP521VerifyingKey = VerifyingKey<NistP521>;

    /// Alias for `Signature<NistP521>` used to represent ecdsa p521 signature.
    pub type EcdsaP521Signature = Signature<NistP521>;
    /// Alias for `DerSignature` used to represent ecdsa p521 der signature.
    pub type EcdsaP521DerSignature = DerSignature;
    /// Alias for `SignatureBytes<NistP521>` used to represent ecdsa p521 signature bytes.
    pub type EcdsaP521SignatureBytes = SignatureBytes<NistP521>;

    /// Creates a cryptographic key from the supplied key material.
    pub fn new_key() -> EcdsaP521PrivateKey {
        use crypto_common::Generate;
        EcdsaP521PrivateKey::generate()
    }

    /// Constructs this value from a coords raw.
    pub fn from_coords_raw(x: &[u8], y: &[u8]) -> Option<EcdsaP521PublicKey> {
        let mut field_x = EcdsaP521FieldBytes::default();
        if x.len() != field_x.len() {
            return None;
        }

        let mut field_y = EcdsaP521FieldBytes::default();
        if y.len() != field_y.len() {
            return None;
        }

        field_x.copy_from_slice(x);
        field_y.copy_from_slice(y);

        let ep = EcdsaP521PublicSec1Point::from_affine_coordinates(&field_x, &field_y, false);

        EcdsaP521PublicKey::from_sec1_point(&ep).into_option()
    }
}

/// Provides nist sp800 108 kdf hmac sha256 behavior for the src component.
pub mod nist_sp800_108_kdf_hmac_sha256 {
    use crypto_common::KeySizeUser;
    use digest::consts::*;
    use hmac::Hmac;
    use kbkdf::Counter;
    use kbkdf::Kbkdf;
    use kbkdf::Params;
    use sha2::Sha256;

    use crate::traits::Zeroizing;

    /// Data carried by mock output operations.
    struct MockOutput;

    impl KeySizeUser for MockOutput {
        /// Alias for `U32` used to represent key size.
        type KeySize = U32;
    }

    /// Alias for `Hmac<Sha256>` used to represent hmac sha256.
    type HmacSha256 = Hmac<Sha256>;

    /// Handles the derive key aes256 behavior for this component.
    pub fn derive_key_aes256(
        key_in: &[u8],
        label: &[u8],
        context: &[u8],
    ) -> Option<Zeroizing<Vec<u8>>> {
        let counter = Counter::<HmacSha256, MockOutput>::default();
        let params = Params::builder(key_in)
            .with_label(label)
            .with_context(context)
            .use_l(true)
            .use_separator(true)
            .use_counter(true)
            .build();
        let key = counter.derive(params).ok()?;

        let mut output = Zeroizing::new(vec![0; MockOutput::key_size()]);
        output.copy_from_slice(key.as_slice());
        Some(output)
    }
}

/// Provides pkcs8 behavior for the src component.
pub mod pkcs8 {
    pub use pkcs8::PrivateKeyInfo;
}

/// Tests behavior implemented by the surrounding module.
#[cfg(test)]
mod tests {
    #[cfg(all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")))]
    use wasm_bindgen_test::*;
    #[cfg(all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")))]
    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    #[cfg_attr(
        all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")),
        wasm_bindgen_test
    )]
    /// Checks that sha256 basic.
    fn sha256_basic() {
        use crate::s256::*;
        use crate::traits::*;

        let mut hasher = Sha256::new();
        hasher.update([0, 1, 2, 3]);
        let out: Sha256Output = hasher.finalize();

        eprintln!("{:?}", out.as_slice());
    }

    #[test]
    #[cfg_attr(
        all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")),
        wasm_bindgen_test
    )]
    /// Checks that hmac 256 basic.
    fn hmac_256_basic() {
        use crate::hmac_s256::*;
        use crate::traits::KeyInit;
        use crate::traits::Mac;

        let hmac_key = new_key();

        let mut hmac = HmacSha256::new(&hmac_key);
        hmac.update(&[0, 1, 2, 3]);
        let out = hmac.finalize();

        eprintln!("{:?}", out.into_bytes());
    }

    #[test]
    #[cfg_attr(
        all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")),
        wasm_bindgen_test
    )]
    /// Checks that hmac 512 basic.
    fn hmac_512_basic() {
        use cipher::KeyInit;

        use crate::hmac_s512::*;

        let hmac_key = new_hmac_sha512_key();

        let mut hmac = HmacSha512::new(&hmac_key);
        hmac.update(&[0, 1, 2, 3]);
        let out = hmac.finalize();

        eprintln!("{:?}", out.into_bytes());
    }

    #[test]
    #[cfg_attr(
        all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")),
        wasm_bindgen_test
    )]
    /// Checks that aes256gcm basic.
    fn aes256gcm_basic() {
        use crate::aes256;
        use crate::aes256gcm::*;

        let aes256gcm_key = aes256::new_key();

        let cipher = Aes256Gcm::new(&aes256gcm_key);

        let nonce = new_nonce();

        // These are the "basic" encrypt/decrypt which postfixs a tag.
        let ciphertext = cipher
            .encrypt(&nonce, b"plaintext message".as_ref())
            .expect("Failed to encrypt message");
        let plaintext = cipher
            .decrypt(&nonce, ciphertext.as_ref())
            .expect("Failed to decrypt message");

        assert_eq!(&plaintext, b"plaintext message");

        // For control of the tag, the following is used.

        // Never re-use nonces
        let nonce = new_nonce();

        let mut buffer = Vec::from(b"test message, super cool");

        // Same as "None"
        let associated_data = b"";

        let tag = cipher
            .encrypt_inout_detached(&nonce, associated_data, buffer.as_mut_slice().into())
            .expect("Failed to encrypt message");

        cipher
            .decrypt_inout_detached(&nonce, associated_data, buffer.as_mut_slice().into(), &tag)
            .expect("Failed to decrypt message");

        assert_eq!(buffer, b"test message, super cool");
    }

    /// Checks that aes256cts basic.
    #[test]
    fn aes256cts_basic() {
        use crate::aes256;
        use crate::aes256cts::*;
        use crate::aes256cts::{self};
        use crate::traits::Generate;

        let key = aes256::new_key();
        let iv = Aes256CtsIv::generate();

        let enc = aes256cts::Aes256CtsEnc::new(&key, &iv);

        let original_buffer = b"plaintext message";
        let mut buffer = *original_buffer;

        enc.encrypt(&mut buffer).expect("encryption failed");

        assert_ne!(&buffer, original_buffer);

        let dec = aes256cts::Aes256CtsDec::new(&key, &iv);

        dec.decrypt(&mut buffer).expect("decryption failed");

        assert_eq!(&buffer, original_buffer);
    }

    #[test]
    #[cfg_attr(
        all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")),
        wasm_bindgen_test
    )]
    /// Checks that aes256cbc basic.
    fn aes256cbc_basic() {
        use crate::aes256;
        use crate::aes256cbc::*;
        use crate::aes256cbc::{self};

        let key = aes256::new_key();
        let iv = aes256cbc::new_iv();

        let enc = aes256cbc::Aes256CbcEnc::new(&key, &iv);

        let ciphertext = enc.encrypt_padded_vec::<block_padding::Pkcs7>(b"plaintext message");

        let dec = aes256cbc::Aes256CbcDec::new(&key, &iv);

        let plaintext = dec
            .decrypt_padded_vec::<block_padding::Pkcs7>(&ciphertext)
            .expect("Unpadding Failed");

        assert_eq!(plaintext, b"plaintext message");
    }

    #[test]
    #[cfg_attr(
        all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")),
        wasm_bindgen_test
    )]
    /// Checks that aes256cbc hmac basic.
    fn aes256cbc_hmac_basic() {
        use crate::aes256;
        use crate::aes256cbc::block_padding;
        use crate::aes256cbc::{self};

        let key = aes256::new_key();

        let (mac, iv, ciphertext) =
            aes256cbc::enc::<block_padding::Pkcs7>(&key, b"plaintext message")
                .expect("Failed to encrypt message");

        let plaintext = aes256cbc::dec::<block_padding::Pkcs7>(&key, &mac, &iv, &ciphertext)
            .expect("Failed to decrypt message");

        assert_eq!(plaintext, b"plaintext message");
    }

    #[test]
    #[cfg_attr(
        all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")),
        wasm_bindgen_test
    )]
    /// Checks that aes256kw basic.
    fn aes256kw_basic() {
        use crate::aes256;
        use crate::aes256kw::*;

        let key_wrap_key = aes256::new_key();
        let key_wrap = Aes256Kw::new(&key_wrap_key);

        let key_to_wrap = aes256::new_key();
        let mut wrapped_key = Aes256KwWrapped::default();

        // Wrap it.
        key_wrap
            .wrap_key(&key_to_wrap, &mut wrapped_key)
            .expect("Failed to wrap key");
        // Reverse the process

        let mut key_unwrapped = aes256::Aes256Key::default();

        key_wrap
            .unwrap_key(&wrapped_key, &mut key_unwrapped)
            .expect("Failed to unwrap key");

        assert_eq!(key_to_wrap, key_unwrapped);
    }

    #[test]
    #[cfg_attr(
        all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")),
        wasm_bindgen_test
    )]
    /// Checks that rsa basic.
    fn rsa_basic() {
        use crate::rsa::*;
        use crate::traits::*;

        let pkey = new_key(MIN_BITS).expect("Failed to generate RSA key");

        let pubkey = RS256PublicKey::from(&pkey);

        // OAEP

        let ciphertext =
            oaep_sha256_encrypt(&pubkey, b"this is a message").expect("Failed to encrypt message");

        let plaintext = oaep_sha256_decrypt(&pkey, &ciphertext).expect("Failed to decrypt message");

        assert_eq!(plaintext, b"this is a message");

        // PKCS1.5 Sig
        let signing_key = RS256SigningKey::new(pkey);
        let verifying_key = RS256VerifyingKey::new(pubkey);

        let mut rng = rand::rng();

        let data = b"Fully sick data to sign mate.";

        let signature = signing_key.sign_with_rng(&mut rng, data);
        assert!(verifying_key.verify(data, &signature).is_ok());

        let signature = signing_key.sign(data);
        assert!(verifying_key.verify(data, &signature).is_ok());
    }

    #[test]
    #[cfg_attr(
        all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")),
        wasm_bindgen_test
    )]
    /// Checks that ecdsa p256 basic.
    fn ecdsa_p256_basic() {
        use crate::ecdsa_p256::*;
        use crate::traits::*;

        let priv_key = new_key();

        let pub_key = priv_key.public_key();

        let signer = EcdsaP256SigningKey::from(&priv_key);
        let verifier = EcdsaP256VerifyingKey::from(&pub_key);

        // Can either sign data directly, using the correct associated hash type.
        let data = [0, 1, 2, 3, 4, 5, 6, 7];

        let sig: EcdsaP256Signature = signer.try_sign(&data).expect("Failed to sign data");

        assert!(verifier.verify(&data, &sig).is_ok());

        // Or you can build the digest content directly, based on the type of the C::Digest value.
        let sig: EcdsaP256Signature = signer
            .try_sign_digest(|digest: &mut EcdsaP256Digest| {
                digest.update(data);
                Ok(())
            })
            .expect("Failed to sign digest");
        assert!(verifier.verify(&data, &sig).is_ok());
    }

    #[test]
    #[cfg_attr(
        all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")),
        wasm_bindgen_test
    )]
    /// Checks that ecdh p256 basic.
    fn ecdh_p256_basic() {
        use crate::ecdh_p256::*;

        let secret_a = new_secret();
        let secret_b = new_secret();

        let public_a = secret_a.public_key();
        let public_b = secret_b.public_key();

        let derived_secret_a = secret_a.diffie_hellman(&public_b);
        let derived_secret_b = secret_b.diffie_hellman(&public_a);

        assert_eq!(
            derived_secret_a.raw_secret_bytes(),
            derived_secret_b.raw_secret_bytes()
        );
    }

    #[test]
    #[cfg_attr(
        all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")),
        wasm_bindgen_test
    )]
    /// Checks that pkcs8 handling test.
    fn pkcs8_handling_test() {
        use pkcs8::PrivateKeyInfoRef;

        use crate::ecdsa_p256;
        use crate::traits::Pkcs8EncodePrivateKey;

        let ecdsa_priv_key = ecdsa_p256::new_key();
        let ecdsa_priv_key_der = ecdsa_priv_key
            .to_pkcs8_der()
            .expect("Failed to encode ECDSA private key");

        let priv_key_info = PrivateKeyInfoRef::try_from(ecdsa_priv_key_der.as_bytes())
            .expect("Failed to parse private key info");

        eprintln!("{priv_key_info:?}");
    }

    /// Checks that rustls mtls basic.
    #[cfg(any(unix, windows))]
    #[test]
    fn rustls_mtls_basic() {
        use std::io::Read;
        use std::io::Write;
        #[cfg(unix)]
        use std::os::unix::net::UnixStream;
        use std::str::FromStr;
        use std::sync::Arc;
        use std::sync::atomic::AtomicU16;
        use std::sync::atomic::Ordering;
        use std::time::Duration;

        use elliptic_curve::SecretKey;
        use rustls::RootCertStore;
        use rustls::client::ClientConfig;
        use rustls::client::ClientConnection;
        use rustls::pki_types::CertificateDer;
        use rustls::pki_types::PrivateKeyDer;
        use rustls::pki_types::PrivatePkcs8KeyDer;
        use rustls::pki_types::ServerName;
        use rustls::server::ServerConfig;
        use rustls::server::ServerConnection;
        use rustls::{self};
        #[cfg(windows)]
        use uds_windows::UnixStream;
        use x509_cert::der::Encode;
        use x509_cert::name::Name;
        use x509_cert::time::Time;

        use crate::test_ca::*;
        use crate::x509::X509Display;

        // ========================
        // CA SETUP

        let now = now();
        let not_before = Time::try_from(now).expect("Failed to convert system time to X509 time");
        let not_after = Time::try_from(now + Duration::new(3600, 0))
            .expect("Failed to convert system time to X509 time");

        let (root_signing_key, root_ca_cert) = build_test_ca_root(not_before, not_after);

        eprintln!("{}", X509Display::from(&root_ca_cert));

        let subject = Name::from_str("CN=localhost").expect("Failed to parse subject name");

        let (server_key, server_csr) = build_test_csr(&subject);

        let server_cert = test_ca_sign_server_csr(
            not_before,
            not_after,
            &server_csr,
            &root_signing_key,
            &root_ca_cert,
        );

        eprintln!("{}", X509Display::from(&server_cert));

        // ========================
        use p384::pkcs8::EncodePrivateKey;
        let server_private_key_pkcs8_der = SecretKey::from(server_key)
            .to_pkcs8_der()
            .expect("Failed to encode server private key");

        let root_ca_cert_der = root_ca_cert
            .to_der()
            .expect("Failed to encode root CA certificate");
        let server_cert_der = server_cert
            .to_der()
            .expect("Failed to encode server certificate");

        let mut ca_roots = RootCertStore::empty();

        ca_roots
            .add(CertificateDer::from(root_ca_cert_der.clone()))
            .expect("Failed to add root CA certificate");

        let server_chain = vec![
            CertificateDer::from(server_cert_der),
            CertificateDer::from(root_ca_cert_der),
        ];

        let server_private_key: PrivateKeyDer =
            PrivatePkcs8KeyDer::from(server_private_key_pkcs8_der.as_bytes().to_vec()).into();

        // let provider = Arc::new(rustls_rustcrypto::provider());
        let provider = Arc::new(rustls::crypto::aws_lc_rs::default_provider());

        let client_tls_config: Arc<_> = ClientConfig::builder_with_provider(provider.clone())
            .with_safe_default_protocol_versions()
            .expect("invalid protocol versions")
            .with_root_certificates(ca_roots)
            .with_no_client_auth()
            .into();

        let server_tls_config: Arc<_> = ServerConfig::builder_with_provider(provider)
            .with_safe_default_protocol_versions()
            .expect("invalid protocol versions")
            .with_no_client_auth()
            .with_single_cert(server_chain, server_private_key)
            .map(Arc::new)
            .expect("bad certificate/key");

        let server_name = ServerName::try_from("localhost").expect("invalid DNS name");

        let (mut server_unix_stream, mut client_unix_stream) =
            UnixStream::pair().expect("Failed to create UnixStream pair");

        let atomic = Arc::new(AtomicU16::new(0));

        let atomic_t = atomic.clone();

        let handle = std::thread::spawn(move || {
            let mut client_connection = ClientConnection::new(client_tls_config, server_name)
                .expect("Failed to create client connection");

            let mut client = rustls::Stream::new(&mut client_connection, &mut client_unix_stream);

            client.write_all(b"hello").expect("Failed to write data");

            while atomic_t.load(Ordering::Relaxed) != 1 {
                std::thread::sleep(std::time::Duration::from_millis(1));
            }

            println!("THREAD DONE");
        });

        let mut server_connection =
            ServerConnection::new(server_tls_config).expect("Failed to create server connection");

        server_connection
            .complete_io(&mut server_unix_stream)
            .expect("Failed to complete TLS handshake");

        server_connection
            .complete_io(&mut server_unix_stream)
            .expect("Failed to complete TLS handshake");

        let mut buf: [u8; 5] = [0; 5];
        server_connection
            .reader()
            .read_exact(&mut buf)
            .expect("Failed to read data");

        assert_eq!(&buf, b"hello");

        atomic.store(1, Ordering::Relaxed);

        // If the thread paniced, this will panic.
        handle.join().expect("Thread panicked");
    }
}
