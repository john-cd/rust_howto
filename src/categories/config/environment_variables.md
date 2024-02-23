# Environment variables

[![dotenvy][dotenvy-badge]][dotenvy]

[`dotenvy`][dotenvy]⮳ supersedes [`dotenv`][dotenv]⮳.

```rust,editable,no_run
{{#include ../../../deps/tests/dotenvy.rs}}
```

To retrieve a single environment variable,

```rust,editable,should_panic
{{#include ../../../deps/tests/env.rs}}
```

[Working with environment variables in Rust][blog-working-with-env-variables]⮳

## Envy

[![envy][envy-badge]][envy]

Envy can deserialize environment variables into type-safe structs.

```toml
[dependencies]
envy = "0.4"
serde = { version = "1.0", features = ["derive"] }
```

```rust,editable,should_panic,noplayground
{{#include ../../../deps/tests/envy.rs}}
```
