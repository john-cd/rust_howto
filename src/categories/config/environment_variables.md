# Environment variables

{{#include environment_variables.incl.md}}

## Dotenvy

[![dotenvy][c-dotenvy-badge]][c-dotenvy]  [![cat-config][cat-config-badge]][cat-config]

[`dotenvy`][c-dotenvy]{{hi:dotenvy}}⮳ supersedes [`dotenv`][c-dotenv]{{hi:dotenv}}⮳.

```rust,no_run
{{#include ../../../deps/tests/dotenvy.rs}}
```

## `std::env`

To retrieve a single environment variable{{hi:Environment variables}},

```rust,should_panic
{{#include ../../../deps/tests/env.rs}}
```

[Working with environment variables in Rust][blog-working-with-env-variables]⮳

## Envy

[![envy][c-envy-badge]][c-envy]  [![cat-config][cat-config-badge]][cat-config]

`envy` can deserialize environment variables into type-safe structs.

```toml
[dependencies]
envy = "0.4"
serde = { version = "1.0", features = ["derive"] }
```

```rust,should_panic,noplayground
{{#include ../../../deps/tests/envy.rs}}
```

## See Also

[![cat-config][cat-config-badge]][cat-config]

[![dotenv][c-dotenv-badge]][c-dotenv]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO: interaction between config and env variables
</div>
