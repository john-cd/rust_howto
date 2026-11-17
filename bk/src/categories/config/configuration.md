<!-- TODO Reviewed ??-??-2026. -->

# Configuration

Layer configuration in a clear precedence order: built-in defaults, configuration files, environment variables, then command-line options. Deserialize the merged result into a typed struct so missing or invalid settings fail during startup.

[`config`][c~config~docs]↗{{hi:config}} supports layered sources and several file formats. [`figment`][c~figment~docs]↗{{hi:figment}} tracks the source and profile for values and errors, and is used by Rocket. [`confique`][c~confique~docs]↗{{hi:confique}} provides typed, Serde-based layered configuration. Use [`dotenvy`][c~dotenvy~docs]↗{{hi:dotenvy}} to load a development `.env` file into the process environment; production environments usually set variables directly.

{{#include configuration.incl.md}}

## Load Typed Settings from Multiple Sources with `config` {#config}

{{#badges config config=Configuration}}{{hi:Environment}}{{hi:Env}}{{hi:Settings}}{{hi:config}}{{hi:Configuration}}

[`config`][c~config~docs]↗{{hi:config}} merges configuration sources in the order they are added, so later sources override earlier values. This recipe applies a default, TOML files, `APP_` environment variables, and a command-line port override, then deserializes the result into typed settings.

The crate also supports other file formats when their features are enabled, along with string sources and programmatic overrides.

```rust,editable
{{#include ../../../crates/cats/config/examples/configuration/config_hierarchical/mod.rs:example}}
```

### Load Configuration Settings into a Singleton {#load_configuration_into_a_singleton}

{{#badges config config=Configuration}}{{hi:Environment}}{{hi:Env}}{{hi:Settings}}{{hi:config}}{{hi:Configuration}}

```rust,editable
{{#include ../../../crates/cats/config/examples/configuration/config_singleton.rs:example}}
```

### Test Configuration-loading Methods {#test_configuration_loading_methods}

{{#badges config config=Configuration}}{{hi:Environment}}{{hi:Env}}{{hi:Settings}}{{hi:config}}{{hi:Configuration}}

```rust,editable
{{#include ../../../crates/cats/config/examples/configuration/config_testing.rs:example}}
```

## Load a TOML Configuration File with `confy` {#confy}

{{#badges confy}}

[`confy`][c~confy~docs]↗{{hi:confy}} loads and stores a Serde configuration at an application-specific path, creating a file with default values when none exists. Use `load_path` when the application needs to select an explicit file path.

```rust,editable,noplayground
{{#include ../../../crates/cats/config/examples/configuration/confy.rs:example}}
```

## Related Topics {#related-topics .skip}

- [Environment variables][p~environment-variables], including `.env` files with `dotenvy`.
- [TOML][p~toml] for the file format used by the layered example.
- [Command-line argument parsing with `clap`][p~cli-argument-parsing] for typed CLI overrides.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
