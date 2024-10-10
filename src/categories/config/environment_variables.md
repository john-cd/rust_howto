# Environment variables

{{#include environment_variables.incl.md}}

## Dotenvy

[![dotenvy][dotenvy-badge]][dotenvy]  [![cat-config][cat-config-badge]][cat-config]

[`{{i:dotenvy}}`][dotenvy]⮳ supersedes [`{{i:dotenv}}`][dotenv]⮳.

```rust,editable,no_run
{{#include ../../../deps/tests/dotenvy.rs}}
```

## `std::env`

To retrieve a single {{i:environment variable}},

```rust,editable,should_panic
{{#include ../../../deps/tests/env.rs}}
```

[Working with environment variables in Rust][blog-working-with-env-variables]⮳

## Envy

[![envy][envy-badge]][envy]  [![cat-config][cat-config-badge]][cat-config]

`Envy` can deserialize environment variables into type-safe structs.

```toml
[dependencies]
envy = "0.4"
serde = { version = "1.0", features = ["derive"] }
```

```rust,editable,should_panic,noplayground
{{#include ../../../deps/tests/envy.rs}}
```

## See Also

[![cat-config][cat-config-badge]][cat-config]

[![dotenv][dotenv-badge]][dotenv]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
