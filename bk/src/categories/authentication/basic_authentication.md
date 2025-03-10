# Basic Authentication

{{#include basic_authentication.incl.md}}

## Perform a basic HTTP authentication {#basic-authentication}

[![reqwest][c-reqwest-badge]][c-reqwest]{{hi:reqwest}} [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]{{hi:Network programming}} [![cat-authentication][cat-authentication-badge]][cat-authentication]{{hi:Authentication}}

Uses [`reqwest::RequestBuilder::basic_auth`][c-reqwest::RequestBuilder::basic_auth]{{hi:reqwest::RequestBuilder::basic_auth}} to perform a basic HTTP authentication{{hi:HTTP authentication}}.

```rust,editable,noplayground
{{#include ../../../crates/cats/authentication/tests/basic.rs:example}}
```

## Related Topics

- [[cryptography | Cryptography]].
- [[encryption | Encryption]].
- [[signature | Signatures]].
- [[certificates | Certificates]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[basic_authentication: expand](https://github.com/john-cd/rust_howto/issues/224) authentication/basic.rs is noplayground because of network use. rewrite?
</div>
