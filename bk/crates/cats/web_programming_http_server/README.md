<!-- TODO Reviewed ??-??-2026 -->

# Web Programming HTTP Server

Examples for building web servers using Actix-web, Axum, Rocket, and gRPC with Tonic. It demonstrates integration with key ecosystem crates including `actix-web`, `anyhow`, `async-graphql`, `async-graphql-axum`, `axum`, `axum-extra`.

## Examples Covered

- **Axum Extras** (`examples/axum_extras/`): Demonstrates concepts like `axum_server_tls`, `cookies`, `debug_handler`, `flash_messages`, `https_redirect`, `typed_header`.
- **Grpc** (`examples/grpc/`): Demonstrates concepts like `echo`, `grpc_web`, `tonic`, `tonic_client`.
- **Loco** (`examples/loco/`): Code examples covering loco.
- **Middleware** (`examples/middleware/`): Demonstrates concepts like `cors`, `tower_http`.
- **Openapi** (`examples/openapi/`): Demonstrates concepts like `schemars`, `utoipa`.
- **Standalone Examples**: `actix_web`, `async_graphql`, `axum`, `hyper_server`, `leptos`, `rocket`.

## Usage

To check or run examples in this crate:

```console
cargo check --package web_programming_http_server --examples
cargo test --package web_programming_http_server
```

Code examples correspond to chapters in `bk/src/categories/web_programming_http_server` or related book sections.
