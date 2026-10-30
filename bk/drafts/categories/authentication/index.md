<!-- TODO Reviewed ??-??-2026 -->

# Authentication

The process of confirming identities.

## Basic Authentication

{{#include basic_authentication.incl.md}}

## Session Login

{{#include session_login.incl.md}}

## OAuth 2.0 and OpenID Connect {#oauth-2-0-and-openid-connect}

OAuth 2.0 is an authorization framework for delegated access to protected APIs. A client requests access for specific scopes and receives an access token from an authorization server. The resource server uses that token to authorize requests. OAuth 2.0 does not define user authentication; OpenID Connect (OIDC) adds an identity layer and ID tokens for sign-in. An access token and an ID token serve different purposes, and an access token is not necessarily a JWT.

### When to Use OAuth or OIDC

Use OAuth when an application needs a user's consent to call an API on the user's behalf. Use OIDC when an application delegates sign-in to an identity provider and needs verified identity claims. For a first-party API, the JWT example below illustrates a local bearer-token format; it does not implement an OAuth authorization server or replace a session design.

### Choose a Client Flow

For interactive clients, use the authorization-code flow with Proof Key for Code Exchange (PKCE). Public clients, including browser and native apps, cannot keep a client secret confidential; use PKCE and validate the authorization response's `state`. Native apps should open the system browser rather than collect credentials in an embedded web view. Follow the current [OAuth 2.0 Security Best Current Practice][rfc~9700], [OAuth 2.0 for Browser-Based Applications][rfc~10017], and [OAuth 2.0 for Native Apps][rfc~8252]. Avoid the implicit and resource-owner-password grants. Register exact redirect URIs, request only needed scopes, and protect access and refresh tokens from logs and untrusted storage.

The [`oauth2` crate][c~oauth2~docs]↗ provides typed OAuth client protocol primitives, including authorization-code and PKCE support; it is not an identity provider or a complete web login system. The [`openidconnect` crate][c~openidconnect~docs]↗ provides OIDC relying-party types and flows. Use your provider's discovery metadata and validate issuer, audience, signature, expiration, and nonce as required by the chosen protocol flow. See the [OpenID Connect Core specification][openid~core] for the identity layer.

For interactive sign-in with a federated identity provider, see the [offline OpenID Connect authorization-code recipe][ex~authentication~oidc-authorization-code]: it shows `state`, `nonce` and PKCE handling and ID-token validation with deterministic fixtures.

For service-to-service access, the client-credentials grant obtains a scoped token for the service itself. See the [offline client-credentials recipe][ex~authentication~oauth-client-credentials] for a `reqwest` request against local mock servers; this grant does not sign in a user.

## JWT Bearer Authentication {#jwt-bearer-authentication}

JSON Web Tokens (JWTs) encode claims in a signed token. The payload is encoded, not encrypted, so do not place secrets in claims. The following local example uses HS256 to issue a short-lived token and verify its signature and `exp` claim. Set `JWT_SECRET` to at least 32 random bytes to run it; do not reuse the example key in production.

### Sign and Verify Claims

[![jsonwebtoken][c~jsonwebtoken~docs~badge]][c~jsonwebtoken~docs]↗{{hi:jsonwebtoken}}

```rust,editable
{{#include ../../../crates/cats/authentication/examples/jwt_authentication/jwt.rs:example}}
```

### Extract a Bearer Token and Check a Role

The Axum extractor reads only the `Authorization: Bearer` header; it does not accept query-string tokens, which can leak through URL logs. It returns 401 for a missing or invalid token, then the route returns 403 when a valid subject lacks the required role. The example installs its key as an Axum extension; keep production signing keys in server-side configuration and never put them in a token or log them.

```rust,editable
{{#include ../../../crates/cats/authentication/examples/jwt_authentication/jwt_axum_extractor.rs:example}}
```

Run the example from `bk/crates` with `cargo run -p authentication --example jwt_authentication`. Its unit tests exercise token expiration, signature tampering, bearer extraction, and role authorization without network access.

## References

The [Copenhagen Book][book~copenhagen]↗ provides a general guideline on implementing Auth in web applications.

## Related Topics

- **[[password_hashing | Password Hashing]]:** [`bcrypt`][c~bcrypt~docs]↗{{hi:bcrypt}}, [`argon2`][c~argon2~docs]↗{{hi:argon2}}, and [`scrypt`][c~scrypt~docs]↗{{hi:scrypt}}. See [[hashing | Hashing]].
- **JWT (JSON Web Tokens):** [`jsonwebtoken`][c~jsonwebtoken~docs]↗{{hi:jsonwebtoken}}.
- **OAuth 2.0:** [![oauth2][c~oauth2~docs~badge]][c~oauth2~docs]↗{{hi:oauth2}} for delegated API authorization.
- **OpenID Connect:** [![openidconnect][c~openidconnect~docs~badge]][c~openidconnect~docs]↗{{hi:openidconnect}} for federated user authentication built on OAuth 2.0.
- **Passkeys (WebAuthn):** [![webauthn-rs][c~webauthn-rs~docs~badge]][c~webauthn-rs~docs]↗{{hi:webauthn-rs}} for phishing-resistant, passwordless sign-in. See the [passkey recipe][ex~authentication~passkeys].
- **Web Authentication:** Often tied to web frameworks such as [`actix-web`][c~actix-web~docs]↗{{hi:actix-web}} and [`warp`][c~warp~docs]↗{{hi:warp}}. See [[web-programming | Web Programming]] and [[web-programming_http-server | Web Programming HTTP Server]].
- **[[cryptography | Cryptographic Primitives]]:** [`ring`][c~ring~docs]↗{{hi:ring}} and [`rust-crypto`][c~rust-crypto~docs]↗{{hi:rust-crypto}} are often used through higher-level crates. See [[encryption | Encryption]], [[signature | Signatures]], [[certificates | Certificates]], and [[cryptography_utilities | Cryptography Utilities]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
