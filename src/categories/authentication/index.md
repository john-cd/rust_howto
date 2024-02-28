# Authentication

The process of confirming identities.

{{#include index.incl.md}}

## Basic Authentication

[![reqwest][reqwest-badge]][reqwest]  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]  [![cat-authentication][cat-authentication-badge]][cat-authentication]

Uses [`reqwest::RequestBuilder::basic_auth`][reqwest::RequestBuilder::basic_auth] to perform a basic HTTP authentication.

```rust,editable,no_run
{{#include ../../../deps/tests/basic.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
