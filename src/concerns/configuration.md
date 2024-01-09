# Configuration

## Environment variables

[dotenvy][dotenvy]⮳ supersedes [dotenv][dotenv]⮳.

```rust,editable,ignore
{{#include ../../deps/examples/dotenvy.rs}}
```

To retrieve a single environment variable,

```rust,editable,should_panic
{{#include ../../deps/examples/env.rs}}
```

[Working with environment variables in Rust][working-with-env-variables]⮳

### Envy

Envy can deserialize environment variables into typesafe struct.

```toml
[dependencies]
envy = "0.4"
serde = { version = "1.0", features = ["derive"] }
```

```rust,editable,ignore
{{#include ../../deps/examples/envy.rs}}
```

## Config

[![config-badge]][config]

`Config` is a layered configuration system for Rust applications.
It reads from JSON, TOML, YAML, INI, RON, JSON5 files.

## Confy

[![confy-badge]][confy]

```rust,editable,ignore
{{#include ../../deps/examples/confy.rs}}
```

{{#include ../link-refs.md}}
