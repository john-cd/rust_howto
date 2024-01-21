# Authentication

## Basic Authentication

[![reqwest-badge]][reqwest] [![cat-network-programming-badge]][cat-network-programming]

Uses [`reqwest::RequestBuilder::basic_auth`][reqwest::RequestBuilder::basic_auth] to perform a basic HTTP authentication.

```rust,editable,no_run
{{#include ../../../deps/examples/basic.rs}}
```

[reqwest::RequestBuilder::basic_auth]: https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html#method.basic_auth
{{#include ../../refs/link-refs.md}}