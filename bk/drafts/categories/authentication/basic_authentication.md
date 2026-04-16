# Basic Authentication

{{#include basic_authentication.incl.md}}

## Perform a Basic HTTP Authentication {#basic-authentication}

[![reqwest][c~reqwest~docs~badge]][c~reqwest~docs]{{hi:reqwest}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}} [![cat~authentication][cat~authentication~badge]][cat~authentication]{{hi:Authentication}}

Uses [`reqwest::RequestBuilder::basic_auth`][c~reqwest::RequestBuilder::basic_auth~docs]↗{{hi:reqwest::RequestBuilder::basic_auth}} to perform a basic HTTP authentication{{hi:HTTP authentication}}.

```rust,editable,noplayground
{{#include ../../../crates/cats/authentication/examples/basic_authentication/basic.rs:example}}
```

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
