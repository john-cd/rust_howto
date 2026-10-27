<!-- TODO Reviewed ??-??-2026 -->

# Other Web Frameworks

This page compares Rocket, Poem, Salvo, and Warp. Axum and Actix Web have dedicated chapters; full-application scaffolding is covered in [Batteries-included Frameworks][p~batteries-included-frameworks], and Hyper is a lower-level HTTP building block.

## Compare Frameworks

| Framework                                                                                                              | Request and routing model                                                                                 | Consider it when                                                                                       |
| ---------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------ |
| [Rocket][c~rocket~docs]↗ [![rocket][c~rocket~docs~badge]][c~rocket~docs]                                               | Declared routes with attribute macros; typed request and data guards validate inputs before handlers run. | You want route signatures and request validation to be explicit and type-driven.                       |
| [Poem][c~poem~docs]↗ [![poem][c~poem~docs~badge]][c~poem~docs] [![poem~repo][c~poem~repo~badge]][c~poem~repo]          | Endpoint and handler functions, extractors, routing, and feature-gated middleware.                        | You want an endpoint-oriented API with modular optional features and test utilities.                   |
| [Salvo][salvo~website]↗ [![salvo][c~salvo~docs~badge]][c~salvo~docs] [![salvo~repo][c~salvo~repo~badge]][c~salvo~repo] | Tree-based routing; handlers also provide middleware, with optional protocol and integration features.    | You want to compose routing and middleware and select optional capabilities such as OpenAPI or HTTP/3. |
| [Warp][c~warp~docs]↗ [![warp][c~warp~docs~badge]][c~warp~docs] [![warp~repo][c~warp~repo~badge]][c~warp~repo]          | Composable `Filter`s combine routing, extraction, and response behavior.                                  | You prefer to build routes from reusable filter pipelines and test them without opening a listener.    |

These frameworks have different APIs and feature sets. Compare their current documentation, supported Rust versions, runtimes, release history, and required integrations before choosing one. For a full-stack framework such as Cot or Loco, see [Batteries-included Frameworks][p~batteries-included-frameworks].

## Recipes

{{#include other_frameworks.incl.md}}

## Implement an HTTP Server with Rocket {#rocket}

[![rocket][c~rocket~docs~badge]][c~rocket~docs]{{hi:rocket}}{{hi:HTTP server}}
[![rocket~crates.io][c~rocket~crates.io~badge]][c~rocket~crates.io]
[![rocket~repo][c~rocket~repo~badge]][c~rocket~repo]
[![rocket~lib.rs][c~rocket~lib.rs~badge]][c~rocket~lib.rs]
[![cat~web-programming::http-server][cat~web-programming::http-server~badge]][cat~web-programming::http-server]{{hi:HTTP server}}

[`rocket`][c~rocket~docs]↗{{hi:rocket}} is an asynchronous web framework with attribute-based routes and request guards. A guard validates request data before Rocket calls a handler, which can keep validation and authorization policies close to the request boundary.

[![rocket~realworld-example~repo][rocket~realworld-example~repo~badge]][rocket~realworld-example~repo]{{hi:rocket}}

[Rust + Rocket RealWorld framework implementation][rocket~realworld-example~repo]↗.

```rust,editable
{{#include ../../../crates/cats/web_programming_http_server/examples/rocket.rs:example}}
```

## Related Topics {#related-topics .skip}

- [Axum][ex~web-programming_http-server~axum] and [Actix Web][ex~web-programming_http-server~actix] have dedicated framework guides.
- [Batteries-included Frameworks][p~batteries-included-frameworks] covers application scaffolding.
- [Hyper][ex~web-programming_http-server~hyper] is a lower-level HTTP implementation.
- [Middleware][p~middleware] and [OpenAPI][p~openapi] add reusable request processing and API documentation.
- Browse the [HTTP server index][p~web-servers] for related recipes.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
