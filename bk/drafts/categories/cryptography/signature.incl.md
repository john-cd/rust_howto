<!-- TODO Reviewed ??-??-2026 -->

Digital signatures let a private-key holder sign data so that a verifier can check its integrity and confirm that the signature was made by someone holding the corresponding key. A signature does not encrypt data or identify a real-world person; applications must establish trust in public keys separately.

For new protocols, Ed25519 with `ed25519-dalek` is a practical default. Choose P-256 ECDSA when interoperability requires it; RSA signatures are mainly for legacy protocol compatibility. The `signature` crate provides common signing and verification traits. Keep private keys protected and use a standard encoding such as [PKCS#8](certificates.md#pkcs8) for storage; sign a well-defined, unambiguous representation of the data.

| Recipe                                           | Crates                                                                             | Categories                                |
| ------------------------------------------------ | ---------------------------------------------------------------------------------- | ----------------------------------------- |
| [`ed25519`][ex~cryptography~ed25519]             | [![ed25519~website][c~ed25519~website~badge]][c~ed25519~website]                   | {{#categories cryptography=Cryptography}} |
| [`ed25519-dalek`][ex~cryptography~ed25519-dalek] | [![ed25519-dalek~website][c~ed25519-dalek~website~badge]][c~ed25519-dalek~website] | {{#categories cryptography=Cryptography}} |
| [`ecdsa`][ex~cryptography~ecdsa]                 | {{#crate ecdsa}}                                                                   | {{#categories cryptography=Cryptography}} |
| [`dsa`][ex~cryptography~dsa]                     | {{#crate dsa}}                                                                     | {{#categories cryptography=Cryptography}} |
