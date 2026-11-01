<!-- TODO Reviewed ??-??-2026 -->

# Protect Secrets

{{#include secrets.incl.md}}

API keys, tokens, and passphrases leak in three common ways: into logs and error messages, through memory that is freed but not erased, and onto disk in plain text. The recipes below address each in turn.

## Threat Model {#threat-model .skip}

These techniques reduce accidental exposure. They do not stop an attacker who can already read your process memory, attach a debugger, or run code as your user. Decide what you defend against before choosing a tool:

- **Accidental disclosure**: a secret printed by `{:?}`, captured in a log line, a panic message, a crash report, or serialized into a response. Redacting types address this.
- **Memory remnants**: freed memory, swap files, and core dumps may still hold a secret. Wiping on drop narrows the window but cannot erase copies.
- **Data at rest**: secrets in source control, configuration files, or project archives. Use the operating system's credential store, or encrypt with a key derived from a passphrase.

Rules of thumb:

- Never log secrets, and never include them in error messages. Make the type system enforce it: a secret type should not implement `Display`, `Debug`, or `Serialize` in a revealing way.
- Prefer the operating system's credential store to files and environment variables.
- Rotate keys and tokens on a schedule and after any suspected leak.
- Do not invent your own primitives. Compose maintained [AEAD][p~aead] and [password hashing][p~password_hashing] crates, as the sealed envelope below does.

## Redact Secrets in `Debug` and `Display` Output {#redacted-secret}

{{#badges secrecy cryptography=Cryptography}}{{hi:Secret}}{{hi:Redact}}{{hi:Logging}}

A newtype whose `Debug` and `Display` implementations print a placeholder lets a configuration struct derive `Debug` without leaking its credentials. Reading the value requires an explicit `expose_secret` call, which is easy to find in a code review. [`secrecy`][c~secrecy~docs]↗{{hi:secrecy}} provides a maintained version of the same idea that also wipes the value on drop.

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/secrets/redacted_secret.rs:example}}
```

## Wipe Secrets from Memory {#zeroize-secrets}

{{#badges zeroize cryptography=Cryptography}}{{hi:Zeroizing}}{{hi:Memory}}{{hi:Drop}}

[`zeroize`][c~zeroize~docs]↗{{hi:zeroize}} overwrites memory in a way the compiler may not optimize away. Derive `Zeroize` and `ZeroizeOnDrop` on a struct that holds a secret, or wrap a value in `Zeroizing<T>`.

Zeroizing has limits: it clears only the memory it owns. A `clone()`, a `String` that reallocated while growing, a value moved by the optimizer, swap, and core dumps may keep earlier copies. Reserve the final capacity up front, avoid cloning secrets, and treat zeroizing as defense in depth.

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/secrets/zeroize.rs:example}}
```

## Store Secrets in the OS Credential Store {#keyring}

{{#badges keyring cryptography=Cryptography}}{{hi:Keychain}}{{hi:Credential Manager}}{{hi:Secret Service}}

[`keyring`][c~keyring~docs]↗{{hi:keyring}} stores, loads, and deletes secrets in Windows Credential Manager, the macOS Keychain, or the Linux Secret Service. The secret never touches your configuration files. The test that talks to the real store is marked `#[ignore]` because CI machines and containers often have none; a second test checks the wrapper without touching the store.

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/secrets/keyring.rs:example}}
```

## Seal a Secret Under a Passphrase {#sealed-envelope}

[![argon2][c~argon2~docs~badge]][c~argon2~docs] [![chacha20poly1305][c~chacha20poly1305~docs~badge]][c~chacha20poly1305~docs]{{hi:Argon2id}}{{hi:XChaCha20-Poly1305}}{{hi:Envelope}}{{hi:AEAD}}{{hi:Passphrase}} [![cat~cryptography][cat~cryptography~badge]][cat~cryptography]{{hi:Cryptography}}

When a secret must be written to a file or shipped in an archive, encrypt it with a key derived from a passphrase. [Argon2id][ex~cryptography~argon2] turns the passphrase and a random salt into a 32-byte key with an explicit memory and time cost. `XChaCha20Poly1305` (see [AEAD][p~aead]) then encrypts the secret under a random 24-byte nonce; its larger nonce makes random nonces safe.

The version label and a caller-supplied context go in as associated data. They are not encrypted, but they are authenticated: moving the envelope to another context, or downgrading its version, makes decryption fail. The envelope stores the Argon2 cost so it can be raised later, and the code that reads it should bound those values so an untrusted file cannot demand unbounded memory. There is no recovery: a lost passphrase means entering the secret again.

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/secrets/sealed_envelope.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[aead | AEAD]].
- [[password_hashing | Password Hashing]].
- [[cryptography_utilities | Cryptographic Utilities]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
