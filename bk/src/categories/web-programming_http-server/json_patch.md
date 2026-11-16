<!-- TODO Reviewed ??-??-2026. -->

# Update JSON Fields with Axum

A PATCH request often needs three states: an omitted field leaves its current value alone, `null` clears it, and a value replaces it. `Option<String>` alone merges the first two states during Serde deserialization. This local [Axum](https://docs.rs/axum/latest/axum/) example uses `#[serde(default)]` for omission and a custom `Deserialize` implementation for explicit null. It validates a nickname before changing an in-memory profile.

```rust
{{#include ../../../crates/cats/web_programming_http_server/examples/json_patch.rs:example}}
```

From `bk/crates`, run `cargo test -p web_programming_http_server --example json_patch`. The tests drive the router with `tower::ServiceExt::oneshot`; no socket, database, or account is needed. The endpoint returns `400` for malformed JSON, `422` for a failed nickname rule, and `500` if its in-memory state is unavailable. Each error has a small JSON body. Production APIs should define an explicit schema and access-control policy for every mutable field. See [OpenAPI and JSON Schema](openapi.md) for generating API documentation and validating JSON.

Other frameworks fit other jobs: [Leptos](https://book.leptos.dev/01_introduction.html) for reactive full-stack UI, [Dropshot](https://github.com/oxidecomputer/dropshot) for typed service APIs, [Pavex](https://github.com/LukeMathWalker/pavex) for a dependency-injection-oriented server framework, [SQLPage](https://crates.io/crates/sqlpage) for SQL-driven pages, and [Smithy](https://smithy.io/) for modeled APIs. Compare the operational requirements of a real application before choosing one.
