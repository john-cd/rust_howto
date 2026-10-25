<!-- TODO Reviewed ??-??-2026 -->

# Basic Authentication

{{#include basic_authentication.incl.md}}

HTTP Basic authentication sends a user name and password in the `Authorization` header. The credentials are Base64-encoded; Base64 is not encryption. Use Basic authentication only over HTTPS, which protects the header in transit.

Treat the credentials as secrets: do not commit them, put them in command-line arguments, or log request headers. The example reads its endpoint and credentials from environment variables so it has no embedded secret. In a deployed application, use the secret-management mechanism provided by the environment.

## Perform a Basic HTTP Authentication {#basic-authentication}

[![reqwest][c~reqwest~docs~badge]][c~reqwest~docs]{{hi:reqwest}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}} [![cat~authentication][cat~authentication~badge]][cat~authentication]{{hi:Authentication}}

[`reqwest::RequestBuilder::basic_auth`][c~reqwest::RequestBuilder::basic_auth~docs]↗{{hi:reqwest::RequestBuilder::basic_auth}} adds the Basic authorization header to a request. The example uses placeholder credentials and builds the request locally; it does not send it, read environment variables, or require a service.

```rust,editable,noplayground
{{#include ../../../crates/cats/authentication/examples/basic_authentication/basic.rs:example}}
```

Its unit test checks that the `Authorization` header is present and correctly encoded without sending a request. If you send a real Basic-authenticated request, use HTTPS, keep credentials in a secret manager, and never log the header. The example is excluded from Rust Playground because the blocking `reqwest` client depends on native TLS support.

## Related Topics {#related-topics .skip}

- [[cryptography | Cryptography]].
- [[encryption | Encryption]].
- [[signature | Signatures]].
- [[certificates | Certificates]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [expand](https://github.com/john-cd/rust_howto/issues/224)
</div>
