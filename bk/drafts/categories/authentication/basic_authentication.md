<!-- TODO Reviewed ??-??-2026 -->

# Basic Authentication

{{#include basic_authentication.incl.md}}

HTTP Basic authentication sends a user name and password in the `Authorization` header. The credentials are Base64-encoded; Base64 is not encryption. Use Basic authentication only over HTTPS, which protects the header in transit.

Treat the credentials as secrets: do not commit them, put them in command-line arguments, or log request headers. This example uses fixed placeholders only; load the endpoint and credentials from a secret-management mechanism in a deployed application.

## Perform a Basic HTTP Authentication {#basic-authentication}

[![reqwest][c~reqwest~docs~badge]][c~reqwest~docs]{{hi:reqwest}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}} [![cat~authentication][cat~authentication~badge]][cat~authentication]{{hi:Authentication}}

[`reqwest::RequestBuilder::basic_auth`][c~reqwest::RequestBuilder::basic_auth~docs]↗{{hi:reqwest::RequestBuilder::basic_auth}} adds the Basic authorization header to a request. The example uses placeholder credentials and builds the request locally; it does not send it, read environment variables, or require a service.

```rust,editable,noplayground
{{#include ../../../crates/cats/authentication/examples/basic_authentication/basic.rs:example}}
```

Its test sends the built request through an in-memory Axum route and verifies that valid credentials receive `200 OK` while invalid credentials receive `401 Unauthorized`. It does not open a socket or contact a service. If you send a real Basic-authenticated request, use HTTPS, keep credentials in a secret manager, and never log the header. The example is excluded from Rust Playground because the blocking `reqwest` client depends on native TLS support.

## Related Topics {#related-topics .skip}

- [[cryptography | Cryptography]].
- [[encryption | Encryption]].
- [[signature | Signatures]].
- [[certificates | Certificates]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  expand
</div>
