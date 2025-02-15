# Environment variables

{{#include environment_variables.incl.md}}

## `dotenvy` {#dotenvy}

[![dotenvy][c-dotenvy-badge]][c-dotenvy] [![dotenvy-crates.io][c-dotenvy-crates.io-badge]][c-dotenvy-crates.io] [![dotenvy-github][c-dotenvy-github-badge]][c-dotenvy-github] [![dotenvy-lib.rs][c-dotenvy-lib.rs-badge]][c-dotenvy-lib.rs]{{hi:dotenvy}}{{hi:Environment}}{{hi:Env}}{{hi:Dotenv}}{{hi:Settings}}{{hi:Config}}

[`dotenvy`][c-dotenvy]{{hi:dotenvy}}⮳ forks and supersedes [`dotenv`][c-dotenv]{{hi:dotenv}}⮳.

```rust,editable,noplayground
{{#include ../../../crates/cats/config/tests/env/dotenvy.rs:example}}
```

## `std::env` {#env}

[![std][c-std-badge]][c-std]{{hi:std}} [![cat-config][cat-config-badge]][cat-config]{{hi:Configuration}}

Use [`std::env`][c-std::env::var]⮳{{hi:std::env}} to retrieve a single environment variable{{hi:Environment variables}}.

```rust,editable,should_panic
{{#include ../../../crates/cats/config/tests/env/env.rs:example}}
```

[Working with environment variables in Rust][blog-working-with-env-variables]⮳

## `envy` {#envy}

[![envy][c-envy-badge]][c-envy] [![envy-crates.io][c-envy-crates.io-badge]][c-envy-crates.io] [![envy-github][c-envy-github-badge]][c-envy-github] [![envy-lib.rs][c-envy-lib.rs-badge]][c-envy-lib.rs]{{hi:envy}}{{hi:Env}}{{hi:Serde}} [![cat-config][cat-config-badge]][cat-config]{{hi:Configuration}}

`envy` can deserialize environment variables into type-safe structs.

```toml
[dependencies]
envy = "0.4"
serde = { version = "1.0.216", features = ["derive"] }
```

```rust,editable,should_panic,noplayground
{{#include ../../../crates/cats/config/tests/env/envy.rs:example}}
```

## See Also

[![dotenv][c-dotenv-badge]][c-dotenv]{{hi:dotenv}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[environment_variables: interaction between config and env variables (P1)](https://github.com/john-cd/rust_howto/issues/271)
</div>
