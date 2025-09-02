# GraphQL

{{#include graphql.incl.md}}

'GraphQL' is a data query language for APIs that allows clients to request only the data they need, avoiding over-fetching or under-fetching. It is intended to serve mobile and web application frontends. Unlike REST APIs, which rely on multiple endpoints, 'GraphQL' uses a single endpoint and a type system to define the structure of the data.

## Create a GraphQL Endpoint {#async-graphql}

[![async-graphql][c~async-graphql~docs~badge]][c~async-graphql~docs]{{hi:async-graphql}}
[![async-graphql~crates.io][c~async-graphql~crates.io~badge]][c~async-graphql~crates.io]
[![async-graphql~repo][c~async-graphql~repo~badge]][c~async-graphql~repo]
[![async-graphql~lib.rs][c~async-graphql~lib.rs~badge]][c~async-graphql~lib.rs]
[![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}
[![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}

[`async-graphql`][c~async-graphql~docs]↗{{hi:async-graphql}} is a high-performance graphql server library that's fully specification compliant. It integrates with [`actix-web`][c~actix-web~docs]↗{{hi:actix-web}}, [`axum`][c~axum~docs]↗{{hi:axum}}, [`poem`][c~poem~docs]↗{{hi:poem}}, [`rocket`][c~rocket~docs]↗{{hi:rocket}}, [`tide`][c~tide~docs]↗{{hi:tide}}, and [`warp`][c~warp~docs]↗{{hi:warp}}.

```rust,editable
{{#include ../../../crates/cats/web_programming_http_server/examples/async_graphql.rs:example}}
```

## Other Options {#other-options .skip}

[![juniper~website][c~juniper~website~badge]][c~juniper~website] [![juniper][c~juniper~docs~badge]][c~juniper~docs] [![juniper~crates.io][c~juniper~crates.io~badge]][c~juniper~crates.io] [![juniper~repo][c~juniper~repo~badge]][c~juniper~repo] [![juniper~lib.rs][c~juniper~lib.rs~badge]][c~juniper~lib.rs]{{hi:juniper}}{{hi:Apollo}}{{hi:Server}}{{hi:Graphql}}{{hi:Web}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}} [![cat~web-programming::http-server][cat~web-programming::http-server~badge]][cat~web-programming::http-server]{{hi:HTTP server}}

The [`juniper`][c~juniper~docs]↗{{hi:juniper}} crate is another Rust implementation of GraphQL, facilitating the creation of GraphQL APIs with strong typing and schema definition. It allows Rust developers to define their API's data structure and query capabilities in a declarative way.

Juniper doesn't include a web server. Instead, it provides building blocks to make integration with existing servers straightforward, including embedded GraphiQL and/or GraphQL Playground for easy debugging.

## Related Topics {#related-topics .skip}

FIXME

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[graphql: write](https://github.com/john-cd/rust_howto/issues/511) integrate in seography graphql section?
</div>
