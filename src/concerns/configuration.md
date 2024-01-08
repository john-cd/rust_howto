# Configuration

## Environment variables

[dotenvy][dotenvy] supersedes [dotenv][dotenv].

```rust,editable,ignore
{{#include ../../deps/examples/dotenvy.rs}}
```

To retrieve a single environment variable,

```rust,editable,should_panic
{{#include ../../deps/examples/env.rs}}
```

[https://www.thorsten-hans.com/working-with-environment-variables-in-rust/]( https://www.thorsten-hans.com/working-with-environment-variables-in-rust/ )

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

[Config][config] is a layered configuration system for Rust applications.
It reads from JSON, TOML, YAML, INI, RON, JSON5 files.

## Confy

[Confy][confy]

```rust,editable,ignore
{{#include ../../deps/examples/confy.rs}}
```

{{#include ../links.md}}
