# Environment variables

{{#include environment_variables.incl.md}}

## Dotenvy

[![dotenvy][c-dotenvy-badge]][c-dotenvy]{{hi:dotenvy}}  [![cat-config][cat-config-badge]][cat-config]{{hi:Configuration}}

[`dotenvy`][c-dotenvy]{{hi:dotenvy}}⮳ supersedes [`dotenv`][c-dotenv]{{hi:dotenv}}⮳.

```rust
{{#include ../../../deps/tests/cats/config/dotenvy.rs}}
```

## `std::env`

To retrieve a single environment variable{{hi:Environment variables}},

```rust,should_panic
{{#include ../../../deps/tests/cats/config/env.rs}}
```

[Working with environment variables in Rust][blog-working-with-env-variables]⮳

## Envy

[![envy][c-envy-badge]][c-envy]{{hi:envy}}  [![cat-config][cat-config-badge]][cat-config]{{hi:Configuration}}

`envy` can deserialize environment variables into type-safe structs.

```toml
[dependencies]
envy = "0.4"
serde = { version = "1.0", features = ["derive"] }
```

```rust,should_panic,noplayground
{{#include ../../../deps/tests/cats/config/envy.rs}}
```

## See Also

[![cat-config][cat-config-badge]][cat-config]{{hi:Configuration}}

[![dotenv][c-dotenv-badge]][c-dotenv]{{hi:dotenv}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO: interaction between config and env variables
</div>
