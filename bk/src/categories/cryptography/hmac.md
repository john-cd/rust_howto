# Message Authentication Code

{{#include hmac.incl.md}}

## Sign and Verify a Message with a HMAC Digest {#hmac}

[![ring][c-ring-badge]][c-ring] [![ring-crates.io][c-ring-crates.io-badge]][c-ring-crates.io] [![ring-github][c-ring-github-badge]][c-ring-github] [![ring-lib.rs][c-ring-lib.rs-badge]][c-ring-lib.rs]{{hi:ring}}{{hi:Crypto}}{{hi:Cryptography}}{{hi:Rand}}{{hi:Rsa}}{{hi:ECC}} [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

An HMAC (Hash-based Message Authentication Code) digest is a type of message [authentication][p-authentication] code (MAC) that combines a cryptographic hash function with a secret key. It's used to verify both the integrity and authenticity of a message.

Note that HMAC (Hash-based Message Authentication Code) uses a shared secret key between two parties (symmetric cryptography). It provides both integrity and authentication. It cannot be used for non-repudiation (proof of origin by a third party). Since both parties have the key, either could have generated the HMAC.

The following example uses [`ring::hmac`][c-ring::hmac]{{hi:ring::hmac}}⮳ to creates a [`ring::signature::Signature`][c-ring::signature::Signature]{{hi:ring::signature::Signature}}⮳ of a string, then verifies the signature{{hi:Signature}} is correct.

```rust,editable
{{#include ../../../crates/cats/cryptography/tests/sign/hmac.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1181)
</div>
