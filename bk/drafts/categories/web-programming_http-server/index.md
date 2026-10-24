<!-- TODO Reviewed ??-??-2026 -->
# Web Serving

[![cat~web-programming::http-server][cat~web-programming::http-server~badge]][cat~web-programming::http-server]{{hi:HTTP server}}

Serve data over HTTP. Start by choosing the level of abstraction that fits the application: a framework supplies routing and request handling, while a lower-level HTTP library leaves more of those choices to you.

This index summarizes options; the framework sections below link to runnable examples and focused guides.

| Need | Examples | What to compare |
| --- | --- | --- |
| Application framework | [`axum`][c~axum~docs]↗, [`actix-web`][c~actix-web~docs]↗, [`rocket`][c~rocket~docs]↗, [`warp`][c~warp~docs]↗ | Routing and extractor model, middleware, runtime, testing support, and the framework's compatibility with the rest of your stack. |
| Other frameworks | [Cot][cot.rs~website]↗, [Salvo][salvo~website]↗ | Cot is batteries-included and built on Axum; Salvo uses handlers for routing and middleware. |
| Lower-level HTTP building block | [`hyper`][c~hyper~docs]↗ | Choose this when you need direct control over HTTP connections and are prepared to compose more of the server yourself. |
| Reusable middleware | [`tower`][c~tower~docs]↗ and [`tower-http`][c~tower-http~docs]↗ | Check which middleware layers your chosen framework supports; axum integrates with Tower services. |
| Full application scaffolding | [`loco`][c~loco-rs~docs]↗ and [Rust on Nails][rust-on-nails~website]↗ | Compare the included project structure and conventions with the services your application actually needs. |

## Choosing a Framework

Before committing to a framework, check its current documentation and release history, supported Rust version, platform and runtime requirements, feature flags, and upgrade policy. Also verify the integration points you need: authentication, database pools, observability, static assets, graceful shutdown, and integration testing. These details vary by framework and version.

Axum is built around Tower's service and middleware ecosystem. Actix Web and Rocket provide their own application APIs and integration patterns. Hyper is a lower-level HTTP implementation that can serve as a building block. Pick based on the APIs and compatibility requirements of your project rather than an unverified popularity or performance ranking.

This guidance has no standalone code example because framework selection depends on a project's constraints; the framework pages below demonstrate their APIs.

## Frameworks

### Axum

{{#include _axum.incl.md}}

### Actix Web

{{#include _actix.incl.md}}

### Batteries-included Frameworks

{{#include _batteries-included_frameworks.incl.md}}

### Other Frameworks

{{#include other_frameworks.incl.md}}

### Low-level Serving with `hyper`

{{#include _hyper.incl.md}}

## Middleware

Middleware is a powerful way to add functionality to your web server.

{{#include middleware.incl.md}}

### Cross-origin Resource Sharing

{{#include cors.incl.md}}

## Static Website Generators

{{#include static_website_generators.incl.md}}

## GraphQL

{{#include _graphql.incl.md}}

## gRPC

{{#include _grpc.incl.md}}

## Related Topics

This section is a navigation list to related how-to chapters.

- [Authentication and authorization][p~authentication].
- [HTTP client APIs][p~http-client].
- [Tracing and diagnostics][p~tracing].
{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
