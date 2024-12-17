# GraphQL

{{#include graphql.incl.md}}

## Create a GraphQL endpoint {#async-graphql}

[![async-graphql][c-async_graphql-badge]][c-async_graphql]{{hi:async-graphql}}
[![async-graphql-crates.io][c-async_graphql-crates.io-badge]][c-async_graphql-crates.io]
[![async-graphql-github][c-async_graphql-github-badge]][c-async_graphql-github]
[![async-graphql-lib.rs][c-async_graphql-lib.rs-badge]][c-async_graphql-lib.rs]
[![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]{{hi:Asynchronous}}
[![cat-network-programming][cat-network-programming-badge]][cat-network-programming]{{hi:Network programming}}

`async-graphql` is a high-performance graphql server library that's fully specification compliant. It integrates with `actix-web`, `axum`, `poem`, `rocket`, `tide`, and `warp`.

```rust,editable
{{#include ../../../deps/tests/categories/web_programming_http_server/async_graphql.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[graphql: write (P1)](https://github.com/john-cd/rust_howto/issues/511)

TODO P2 integrate in seography graphql section?
</div>
