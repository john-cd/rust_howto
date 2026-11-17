<!-- TODO Reviewed ??-??-2026. -->

# Environment Variables

{{#include environment_variables.incl.md}}

## `dotenvy` {#dotenvy}

{{#badges dotenvy}}{{hi:Environment}}{{hi:Env}}{{hi:Dotenv}}{{hi:Settings}}{{hi:Config}}

[`dotenvy`][c~dotenvy~docs]↗{{hi:dotenvy}} is a maintained successor to [`dotenv`][c~dotenv~docs]↗{{hi:dotenv}}.

[`dotenvy`][c~dotenvy~docs]↗{{hi:dotenvy}} loads `NAME=value` pairs from a `.env` file into the process environment, which is useful during local development. Keep secret-bearing files out of source control and avoid printing all environment variables: they may contain credentials. The example reads a single setting from an isolated fixture; a deployed application should usually receive environment variables from its runtime platform.

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

{{#badges envy config=Configuration}}{{hi:Env}}{{hi:Serde}}

[`envy`][c~envy~docs]↗{{hi:envy}} can deserialize environment variables into type-safe [structs][p~structs].

```toml
[dependencies]
envy = "0.4"
serde = { version = "1.0.229", features = ["derive"] }
```

```rust,editable,noplayground
{{#include ../../../crates/cats/config/examples/env/envy.rs:example}}
```

## See Also {#see-also .skip}

[![dotenv][c~dotenv~docs~badge]][c~dotenv~docs]{{hi:dotenv}}

## Related Topics {#related-topics .skip}

- [[configuration | Configuration]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  environment_variables: interaction between [config][p~config] and env variables
</div>
