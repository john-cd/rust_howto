# Basic Authentication

{{#include basic_authentication.incl.md}}

## Perform a basic HTTP authentication {#basic-authentication}

[![reqwest][c-reqwest-badge]][c-reqwest]{{hi:reqwest}}  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]{{hi:Network programming}}  [![cat-authentication][cat-authentication-badge]][cat-authentication]{{hi:Authentication}}

Uses [`reqwest::RequestBuilder::basic_auth`][c-reqwest::RequestBuilder::basic_auth]{{hi:reqwest::RequestBuilder::basic_auth}} to perform a basic HTTP authentication{{hi:HTTP authentication}}.

```rust,editable
{{#include ../../../deps/tests/cats/authentication/basic.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO: expand
</div>
