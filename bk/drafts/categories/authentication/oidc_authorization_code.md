<!-- TODO Reviewed ??-??-2026 -->

# OpenID Connect Authorization Code with PKCE

{{#include oidc_authorization_code.incl.md}}

OpenID Connect (OIDC) adds an identity layer to OAuth 2.0: after the user authenticates with a provider, the application receives a signed **ID token** that says who signed in. This recipe is the relying party (the application) side of the authorization-code flow, using the [`openidconnect`][c~openidconnect~docs]↗ crate. It runs without a browser, a provider account or a network, so every step is testable.

## Sign in with the OpenID Connect authorization-code flow and PKCE {#oidc-authorization-code}

[![openidconnect][c~openidconnect~docs~badge]][c~openidconnect~docs] [![openidconnect~crates.io][c~openidconnect~crates.io~badge]][c~openidconnect~crates.io] [![openidconnect~repo][c~openidconnect~repo~badge]][c~openidconnect~repo] [![openidconnect~lib.rs][c~openidconnect~lib.rs~badge]][c~openidconnect~lib.rs]{{hi:openidconnect}}{{hi:OpenID Connect}}{{hi:OIDC}}{{hi:PKCE}} [![cat~authentication][cat~authentication~badge]][cat~authentication]{{hi:Authentication}}

The flow has four steps. The application builds an authorization URL that carries a random `state`, a random `nonce` and a PKCE challenge, and keeps the matching secrets for that one login. The callback is rejected unless its `state` matches. The authorization code is then exchanged together with the PKCE verifier, which proves that the party redeeming the code is the one that asked for it. Finally the ID token is verified: signature, issuer, audience, expiry and the nonce. The access token, by contrast, is opaque to the client; it is presented to the provider's APIs and never parsed or trusted for identity.

```rust,editable,noplayground
{{#include ../../../crates/cats/authentication/examples/oidc_authorization_code/oidc.rs:example}}
```

The tests cover the successful login and then break one thing at a time: a foreign `state`, a nonce from another request, a different issuer or audience, an expired token, a bad signature and a wrong PKCE verifier. Each must be rejected.

Notes for real deployments:

- Take the issuer from your own configuration and use `CoreProviderMetadata::discover` (which also fetches the provider's signing keys from its JWKS URL) only against that trusted issuer. The fixture signs with HS256 and the client secret to stay self-contained; production providers normally use RS256 or ES256.
- Register exact redirect URIs, use HTTPS, never follow redirects from the token endpoint, and set timeouts on the HTTP client.
- Store `state`, `nonce` and the PKCE verifier server-side (or in a signed, short-lived, HttpOnly cookie) and discard them after one use.
- Load the client secret from a secret manager. Public clients (browser and native apps) have no secret and rely on PKCE alone.
- Run a real-provider sign-in manually if you adapt this recipe; those runs are not part of the tests.

## Related Topics {#related-topics .skip}

- [[authentication | Authentication]] for OAuth and OIDC flow choices.
- [[oauth_client_credentials | OAuth 2.0 Client Credentials]] for service-to-service tokens.
- [[session_login | Session Login]] for keeping the user signed in afterwards.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
