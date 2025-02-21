# Authentication

The process of confirming identities.

## Basic Authentication

{{#include basic_authentication.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[P2 add Oauth](https://github.com/john-cd/rust_howto/issues/636)?

Link to Password Hashing: `bcrypt`, `argon2`, `scrypt`
JWT (JSON Web Tokens): `jsonwebtoken`
OAuth 2.0: `oauth2`
Web Authentication: (Often tied to web frameworks like actix-web, warp, etc. No single dominant crate.)
Cryptographic Primitives: `ring`, `rust-crypto` (for underlying crypto, but often abstracted by higher-level crates)

The Copenhagen Book provides a general guideline on implementing auth in web applications.
The [Copenhagen Book][book-copenhagen]
</div>
