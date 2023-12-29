# Configuration

## Environment variables

[dotenvy]( https://crates.io/crates/dotenvy )

It supersedes [dotenv]( https://crates.io/crates/dotenv )

```rust,editable,ignore
{{#include ../../deps/examples/dotenvy.rs}}
```

To retrieve a single environment variable

```rust,editable,,should_panic
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

[config]( https://crates.io/crates/config ): layered configuration system for Rust applications. Read from JSON, TOML, YAML, INI, RON, JSON5 files.

## Confy

[Confy]( https://docs.rs/confy/latest/confy/index.html )

```rust,editable,ignore
{{#include ../../deps/examples/confy.rs}}
```
