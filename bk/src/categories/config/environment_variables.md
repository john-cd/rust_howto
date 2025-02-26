# Environment variables

{{#include environment_variables.incl.md}}

## `dotenvy` {#dotenvy}

[![dotenvy][c-dotenvy-badge]][c-dotenvy] [![dotenvy-crates.io][c-dotenvy-crates.io-badge]][c-dotenvy-crates.io] [![dotenvy-github][c-dotenvy-github-badge]][c-dotenvy-github] [![dotenvy-lib.rs][c-dotenvy-lib.rs-badge]][c-dotenvy-lib.rs]{{hi:dotenvy}}{{hi:Environment}}{{hi:Env}}{{hi:Dotenv}}{{hi:Settings}}{{hi:Config}}

[`dotenvy`][c-dotenvy]{{hi:dotenvy}}⮳ forks and supersedes [`dotenv`][c-dotenv]{{hi:dotenv}}⮳.

[`dotenvy`][c-dotenvy]⮳{{hi:dotenvy}} is a Rust crate that loads environment variables from a .env file. It's commonly used during development to manage configuration settings without hardcoding them into the application. `dotenvy` parses the .env file, sets the environment variables, and makes them accessible to the application through the standard std::env module. It's a simple but effective way to separate [configuration][p-configuration] from code and manage different environments (development, testing, production) with different .env files. It's important to note that .env files are typically not used in production environments, where environment variables are usually set directly.

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

[`envy`][c-envy]⮳{{hi:envy}} can deserialize environment variables into type-safe [structs][p-structs].

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
[environment_variables: interaction between [config][p-config] and env variables (P1)](https://github.com/john-cd/rust_howto/issues/271)
</div>
