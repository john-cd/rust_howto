# Authentication

The process of confirming identities.

## Basic Authentication

{{#include basic_authentication.incl.md}}

## Related Topics

| Topic | Rust Crates | Notes |
|---|---|---|
| [[password_hashing | Password Hashing]] | [`bcrypt`][c~bcrypt~docs]↗{{hi:bcrypt}}, [`argon2`][c~argon2~docs]↗{{hi:argon2}}, [`scrypt`][c~scrypt~docs]↗{{hi:scrypt}}| [[hashing | Hashing]] |
| JWT (JSON Web Tokens) | [`jsonwebtoken`][c~jsonwebtoken~docs]↗{{hi:jsonwebtoken}} | |
| OAuth 2.0 | [`oauth2`][c~oauth2~docs]↗{{hi:oauth2}} | |
| Web Authentication | Often tied to web frameworks like [`actix-web`][c~actix_web~docs]↗{{hi:actix-web}}, [`warp`][c~warp~docs]↗{{hi:warp}}, etc. No single dominant crate. | See [[web-programming | Web Programming]] and [[web-programming_http-server | Web Programming HTTP Server]]. |
| [[cryptography | Cryptographic]] Primitives | [`ring`][c~ring~docs]↗{{hi:ring}}, [`rust-crypto`][c~rust_crypto~docs]↗{{hi:rust-crypto}} (often abstracted by higher-level crates). | See [[encryption | Encryption]], [[signature | Signatures]], [[certificates | Certificates]] and [[cryptography_utilities | Cryptography Utilities]] |

## References

The [Copenhagen Book][book~copenhagen] provides a general guideline on implementing Auth in web applications.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[add Oauth](https://github.com/john-cd/rust_howto/issues/636)

- [jsonwebtoken — Rust auth library](https://lib.rs/crates/jsonwebtoken)
- [rauthy: OpenID Connect Single Sign-On Identity & Access Management](https://github.com/sebadob/rauthy)

- [Passwords](https://lib.rs/crates/passwords)

</div>
