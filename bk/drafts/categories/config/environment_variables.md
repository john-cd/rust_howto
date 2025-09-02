# Environment Variables

{{#include environment_variables.incl.md}}

## `dotenvy` {#dotenvy}

[![dotenvy][c~dotenvy~docs~badge]][c~dotenvy~docs] [![dotenvy~crates.io][c~dotenvy~crates.io~badge]][c~dotenvy~crates.io] [![dotenvy~repo][c~dotenvy~repo~badge]][c~dotenvy~repo] [![dotenvy~lib.rs][c~dotenvy~lib.rs~badge]][c~dotenvy~lib.rs]{{hi:dotenvy}}{{hi:Environment}}{{hi:Env}}{{hi:Dotenv}}{{hi:Settings}}{{hi:Config}}

[`dotenvy`][c~dotenvy~docs]↗{{hi:dotenvy}} forks and supersedes [`dotenv`][c~dotenv~docs]↗{{hi:dotenv}}.

[`dotenvy`][c~dotenvy~docs]↗{{hi:dotenvy}} is a Rust crate that loads environment variables from a .env file. It's commonly used during development to manage configuration settings without hardcoding them into the application. [`dotenvy`][c~dotenvy~docs]↗{{hi:dotenvy}} parses the .env file, sets the environment variables, and makes them accessible to the application through the standard std::env module. It's a simple but effective way to separate [configuration][p~configuration] from code and manage different environments (development, testing, production) with different .env files. It's important to note that .env files are typically not used in production environments, where environment variables are usually set directly.

```rust,editable,noplayground
{{#include ../../../crates/cats/config/examples/env/dotenvy.rs:example}}
```

## `std::env` {#env}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~config][cat~config~badge]][cat~config]{{hi:Configuration}}

Use [`std::env`][c~std::env::var~docs]↗{{hi:std::env}} to retrieve a single environment variable{{hi:Environment variables}}.

```rust,editable
{{#include ../../../crates/cats/config/examples/env/env.rs:example}}
```

[Working with environment variables in Rust][blog~working-with-env-variables]↗.

## `envy` {#envy}

[![envy][c~envy~docs~badge]][c~envy~docs] [![envy~crates.io][c~envy~crates.io~badge]][c~envy~crates.io] [![envy~repo][c~envy~repo~badge]][c~envy~repo] [![envy~lib.rs][c~envy~lib.rs~badge]][c~envy~lib.rs]{{hi:envy}}{{hi:Env}}{{hi:Serde}} [![cat~config][cat~config~badge]][cat~config]{{hi:Configuration}}

[`envy`][c~envy~docs]↗{{hi:envy}} can deserialize environment variables into type-safe [structs][p~structs].

```toml
[dependencies]
envy = "0.4"
serde = { version = "1.0.217", features = ["derive"] }
```

```rust,editable,noplayground
{{#include ../../../crates/cats/config/examples/env/envy.rs:example}}
```

## Related Topics {#related-topics .skip}

FIXME

## See Also {#see-also .skip}

[![dotenv][c~dotenv~docs~badge]][c~dotenv~docs]{{hi:dotenv}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[environment_variables: interaction between [config][p~config] and env variables](https://github.com/john-cd/rust_howto/issues/271)
</div>
