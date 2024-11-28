# Environment variables

{{#include environment_variables.incl.md}}

## Dotenvy {#dotenvy}

[![dotenvy][c-dotenvy-badge]][c-dotenvy]{{hi:dotenvy}} [![cat-config][cat-config-badge]][cat-config]{{hi:Configuration}}

[`dotenvy`][c-dotenvy]{{hi:dotenvy}}⮳ supersedes [`dotenv`][c-dotenv]{{hi:dotenv}}⮳.

```rust,editable
{{#include ../../../deps/tests/cats/config/dotenvy.rs:example}}
```

## `std::env` {#env}

To retrieve a single environment variable{{hi:Environment variables}},

```rust,editable,should_panic
{{#include ../../../deps/tests/cats/config/env.rs:example}}
```

[Working with environment variables in Rust][blog-working-with-env-variables]⮳

## Envy {#envy}

[![envy][c-envy-badge]][c-envy]{{hi:envy}} [![cat-config][cat-config-badge]][cat-config]{{hi:Configuration}}

`envy` can deserialize environment variables into type-safe structs.

```toml
[dependencies]
envy = "0.4"
serde = { version = "1.0", features = ["derive"] }
```

```rust,editable,should_panic,noplayground
{{#include ../../../deps/tests/cats/config/envy.rs:example}}
```

## See Also

[![cat-config][cat-config-badge]][cat-config]{{hi:Configuration}}

[![dotenv][c-dotenv-badge]][c-dotenv]{{hi:dotenv}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO P1 interaction between config and env variables
</div>
