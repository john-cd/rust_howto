<!-- TODO Reviewed ??-??-2026 -->

# Authentication

This crate contains code examples for the **Authentication** category on crates.io. It demonstrates integration with key ecosystem crates including `anyhow`, `argon2`, `axum`, `axum-login`, `getrandom`, `http-body-util`.

## Examples Covered

- **Basic Authentication** (`examples/basic_authentication/`): Demonstrates concepts like `basic`.
- **Jwt Authentication** (`examples/jwt_authentication/`): Demonstrates concepts like `jwt`, `jwt_axum_extractor`.
- **Oauth Client Credentials** (`examples/oauth_client_credentials/`): Demonstrates concepts like `client_credentials`.
- **Session Login** (`examples/session_login/`): Demonstrates concepts like `axum_login`, `csrf`, `hashing`.

## Usage

To check or run examples in this crate:

```console
cargo check --package authentication --examples
cargo test --package authentication
```

Code examples correspond to chapters in `bk/src/categories/authentication` or related book sections.
