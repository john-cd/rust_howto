# Axum

{{#include axum.incl.md}}

## Create a web Server with `axum` {#axum}

[![axum][c~axum~docs~badge]][c~axum~docs] [![axum~crates.io][c~axum~crates.io~badge]][c~axum~crates.io] [![axum~repo][c~axum~repo~badge]][c~axum~repo] [![axum~lib.rs][c~axum~lib.rs~badge]][c~axum~lib.rs]{{hi:axum}}{{hi:Framework}}{{hi:Http}}{{hi:Web}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}} [![cat~web-programming::http-server][cat~web-programming::http-server~badge]][cat~web-programming::http-server]{{hi:HTTP server}}

[`axum`][c~axum~docs]↗{{hi:axum}} is a web framework that focuses on ergonomics and modularity. It is an official part of the [`tokio`][c~tokio~docs]↗{{hi:tokio}} project. [`axum`][c~axum~docs]↗{{hi:axum}} is recommended for most new projects.

```rust,editable
{{#include ../../../crates/cats/web_programming_http_server/examples/axum.rs:example}}
```

## See Also

- [Axum examples][c~axum~examples]↗.
- ["Real world" examples with `axum` and `sqlx`][realworld~rust-axum-sqlx~repo]↗.
- Explore [`realworld.how`][realworld.how]↗ for additional examples.
- [`crates.io` example source code (using Axum)][crates.io~example-source-code~repo]↗.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[axum: write](https://github.com/john-cd/rust_howto/issues/507)

[Rust Axum clean demo][clean_axum_demo~repo]↗ is a one-stop, production-ready API template.

**Rust Axum Clean Demo** is a GitHub template that brings together almost all the best practices and features you need for building a production-ready API server with [Axum][c~axum~repo]↗ and [SQLx][c~sqlx~repo]↗.

While Axum's official examples do a fantastic job of demonstrating individual features (routing, extractors, middleware, etc.), I found it really helpful to have everything wired up in **one** cohesive structure:

- **Domain-first modularity**: each domain (user, auth, device, file...) lives in its own module, with controllers, DB layer, and models neatly organized
- **Clean Architecture** & dependency inversion via traits,
- **SQLx** for compile-time checked queries + offline mode setup,
- **JWT-based auth** (login endpoint + `Extension<Claims>`),
- **File upload & protected file serving** with multipart extractors,
- **Swagger UI docs** powered by `utoipa-swagger-ui` (Authorize, try out endpoints in-browser),
- **Database seeding** scripts to spin up your schema & seed data,
- **Unit & integration tests** out of the box,

- Example on how to [implement OpenTelemetry for Axum][implement-OpenTelemetry-for-Axum~repo]↗.
- Database migrations: review [axum-rest-api-sample][axum-rest-api-sample~repo]↗.

Other Axum template:

- [keterrest][keterrest~repo]↗.

Other Generators:

- [protypo~repo]↗.
- [gerust~website]↗.

</div>
