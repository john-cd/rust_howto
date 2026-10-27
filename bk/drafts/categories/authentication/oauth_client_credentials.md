<!-- TODO Reviewed ??-??-2026 -->

# OAuth 2.0 Client Credentials

{{#include oauth_client_credentials.incl.md}}

The client-credentials grant is for a confidential service acting on its own behalf when calling an API. It does not sign in a person or establish a user identity; use OpenID Connect (OIDC) when an application needs federated sign-in. This example uses the [`oauth2`][c~oauth2~docs]↗ and [`reqwest`][c~reqwest~docs]↗ crates with local mock servers, so it runs without a provider account or external network.

## Request a scoped token and call a protected resource {#oauth-client-credentials}

The example sends the client ID and secret to a local mock authorization server, requests the narrow `records:read` scope, then presents the resulting bearer token to a separate local resource server. The mock token expires after one hour so the example can also show the token lifetime returned by the server.

```rust,editable,noplayground
{{#include ../../../crates/cats/authentication/examples/oauth_client_credentials/client_credentials.rs:example}}
```

In a deployed service, load the client secret from a secret manager, never commit it or log it, and use HTTPS for both endpoints. Treat access tokens as secrets. Cache a token only until its returned expiry and request another when needed; do not assume a fixed lifetime. Request only the scopes the service needs. The placeholder values and local mock HTTP endpoints in this runnable example are not production credentials or endpoints.

The client-credentials grant is not a user-login flow: the access token authorizes the service as a client, and it does not assert who a human user is. Use OIDC when verified identity claims are needed. This original example was inspired by the [Rust Cookbook OAuth 2.0 issue #827][rust-cookbook~oauth2-827]; it does not reuse that project's code or prose.

## Related Topics {#related-topics .skip}

- [[authentication | Authentication]] for OAuth and OIDC flow choices.
- [[session_login | Session Login]] for browser user sign-in.
- [[web-programming_http-client | HTTP Clients]] for `reqwest` requests.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
