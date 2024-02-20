# Configuration

Facilitate configuration management for applications.

## Environment variables

[![dotenvy-badge]][dotenvy]

[`dotenvy`][dotenvy]⮳ supersedes [`dotenv`][dotenv]⮳.

```rust,editable,no_run
{{#include ../../deps/tests/dotenvy.rs}}
```

To retrieve a single environment variable,

```rust,editable,should_panic
{{#include ../../deps/tests/env.rs}}
```

[Working with environment variables in Rust][blog-working-with-env-variables]⮳

### Envy

[![envy-badge]][envy]

Envy can deserialize environment variables into typesafe struct.

```toml
[dependencies]
envy = "0.4"
serde = { version = "1.0", features = ["derive"] }
```

```rust,editable,should_panic,noplayground
{{#include ../../deps/tests/envy.rs}}
```

## Config

[![config-badge]][config]

`Config` is a layered configuration system for Rust applications. It reads from JSON, TOML, YAML, INI, RON, JSON5 files.

## Confy

[![confy-badge]][confy]

```rust,editable,no_run
{{#include ../../deps/tests/confy.rs}}
```

## See also

[![dotenv-badge]][dotenv]

{{#include ../refs/link-refs.md}}
