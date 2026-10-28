<!-- TODO Reviewed ??-??-2026 -->

# Cryptography

Book code examples for Cryptography including AES, RSA, and TLS implementations. It demonstrates integration with key ecosystem crates including `aes-gcm`, `aes-gcm-siv`, `anyhow`, `argon2`, `base64`, `base64ct`.

## Examples Covered

- **Aead** (`examples/aead/`): Demonstrates concepts like `aes_gcm_siv`.
- **Certs** (`examples/certs/`): Demonstrates concepts like `der`, `parse_x509`, `pem_rfc7468`, `pkcs8`, `rcgen`, `read_pem`, `x509_cert`.
- **Encryption** (`examples/encryption/`): Demonstrates concepts like `aes_gcm`, `chacha20poly1305`.
- **Password Hashing** (`examples/password_hashing/`): Demonstrates concepts like `argon2`, `bcrypt`, `pbkdf2`, `scrypt`.
- **Secrets** (`examples/secrets/`): Demonstrates concepts like `keyring`, `redacted_secret`, `sealed_envelope`, `zeroize`.
- **Sign** (`examples/sign/`): Demonstrates concepts like `dsa`, `ecdsa`, `ed25519`, `ed25519_dalek`, `hmac`.
- **Tls** (`examples/tls/`): Demonstrates concepts like `mtls`, `native_tls`, `rustls`, `tokio_rustls`.
- **Utilities** (`examples/utilities/`): Demonstrates concepts like `base64_example`, `getrandom_example`, `hex_example`, `subtle`, `zeroize`.
- **Standalone Examples**: `rsa`.

## Usage

To check or run examples in this crate:

```console
cargo check --package cryptography --examples
cargo test --package cryptography
```

Code examples correspond to chapters in `bk/src/categories/cryptography` or related book sections.
