# Authentication

The process of confirming identities.

## Basic Authentication

{{#include basic_authentication.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[P2 add Oauth](https://github.com/john-cd/rust_howto/issues/636)?

Password Hashing: [`bcrypt`][c-bcrypt]⮳{{hi:bcrypt}}, [`argon2`][c-argon2]⮳{{hi:argon2}}, [`scrypt`][c-scrypt]⮳{{hi:scrypt}}
[[hashing | Hashing]]
[[password_hashing | Password Hashing]]

JWT (JSON Web Tokens): [`jsonwebtoken`][c-jsonwebtoken]⮳{{hi:jsonwebtoken}}

OAuth 2.0: [`oauth2`][c-oauth2]⮳{{hi:oauth2}}

Web Authentication: (Often tied to web frameworks like actix-web, warp, etc. No single dominant crate.)
[[web-programming | Web Programming]]
[[web-programming_http-server | Web Programming HTTP Server]]

Cryptographic Primitives: [`ring`][c-ring]⮳{{hi:ring}}, [`rust-crypto`][c-rust_crypto]⮳{{hi:rust-crypto}} (for underlying crypto, but often abstracted by higher-level crates)
[[cryptography | Cryptography]]
[[encryption | Encryption]]
[[signature | Signature]]
[[certificates | Certificates]]
[[cryptography_utilities | Cryptography Utilities]]

The Copenhagen Book provides a general guideline on implementing auth in web applications.
The [Copenhagen Book][book-copenhagen]
</div>
