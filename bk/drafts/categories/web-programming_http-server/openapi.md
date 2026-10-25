<!-- TODO Reviewed ??-??-2026 -->

# OpenAPI and JSON Schema

{{#include openapi.incl.md}}

Describe an HTTP API and its data types in a machine-readable form, so that documentation, clients, and validators stay in step with the code.

## Generate an OpenAPI Document with `utoipa` {#utoipa}

[![utoipa][c~utoipa~docs~badge]][c~utoipa~docs]{{hi:utoipa}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:web-programming}} [![cat~web-programming::http-server][cat~web-programming::http-server~badge]][cat~web-programming::http-server]{{hi:web-programming::http-server}}

[`utoipa`][c~utoipa~docs]↗{{hi:utoipa}} generates an OpenAPI document from your Rust code (the code-first approach). Derive `ToSchema` for request and response types, annotate each handler with `#[utoipa::path]` to describe its route, parameters, and responses, and collect them with `#[derive(OpenApi)]`. Doc comments become descriptions.

```rust,editable,noplayground
{{#include ../../../crates/cats/web_programming_http_server/examples/openapi/utoipa.rs:example}}
```

[`utoipa-swagger-ui`][c~utoipa-swagger-ui~docs]↗{{hi:utoipa-swagger-ui}} hosts an interactive Swagger UI page next to your API: `SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi())` serves both the UI and the generated JSON, and merges into the router like any other `axum` route set. Its `vendored` feature embeds the UI files instead of downloading them during the build, which keeps offline and reproducible builds working.

The tests render the document with `ApiDoc::openapi().to_pretty_json()` and assert that the path and schema exist, then call the router in memory to check that the JSON route and the UI page respond with `200 OK`, so no server is needed.

Keep the specification honest: commit a generated copy and fail the continuous-integration build when the freshly generated JSON differs, so API changes are reviewed on purpose. Choose between code-first (as here) and spec-first (write the OpenAPI document, then generate code) once per project, and version breaking changes under a new path prefix.

## Derive and Validate a JSON Schema {#schemars}

[![schemars][c~schemars~docs~badge]][c~schemars~docs]{{hi:schemars}} [![jsonschema][c~jsonschema~docs~badge]][c~jsonschema~docs]{{hi:jsonschema}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:web-programming}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:encoding}}

[`schemars`][c~schemars~docs]↗{{hi:schemars}} derives a [JSON Schema](https://json-schema.org) from a type that implements `serde`'s traits, honouring `#[serde(...)]` attributes and doc comments. [`jsonschema`][c~jsonschema~docs]↗{{hi:jsonschema}} validates a JSON value against a schema. Together they let you publish a schema for a configuration file, or reject malformed input with a precise message before deserializing it.

```rust,editable,noplayground
{{#include ../../../crates/cats/web_programming_http_server/examples/openapi/schemars.rs:example}}
```

The tests check that a valid configuration passes, that a wrongly typed or incomplete one is rejected, and that the schema lists the expected properties.

## Related Topics {#related-topics .skip}

- [[axum | Axum]].
- [[grpc | gRPC]] for describing services with Protocol Buffers instead.
- [[web-programming_http-server | HTTP Servers]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[written for issue #1916](https://github.com/john-cd/rust_howto/issues/1916)
</div>
