<!-- TODO Reviewed ??-??-2026 -->

# Configuration

{{#include configuration.incl.md}}

## Load Typed Settings from Multiple Sources with `config` {#config}

[![config][c~config~docs~badge]][c~config~docs] [![config~crates.io][c~config~crates.io~badge]][c~config~crates.io] [![config~repo][c~config~repo~badge]][c~config~repo] [![config~lib.rs][c~config~lib.rs~badge]][c~config~lib.rs]{{hi:config}}{{hi:Environment}}{{hi:Env}}{{hi:Settings}}{{hi:config}}{{hi:Configuration}} [![cat~config][cat~config~badge]][cat~config]{{hi:Configuration}}

[`config`][c~config~docs]↗{{hi:config}} merges configuration sources in the order they are added, so later sources override earlier values. This recipe applies a default, TOML files, `APP_` environment variables, and a command-line port override, then deserializes the result into typed settings.

- JSON, TOML, YAML, INI, and other supported file formats.
- Environment variables, string sources, and programmatic overrides.

```rust,editable
{{#include ../../../crates/cats/config/examples/configuration/config_hierarchical/mod.rs:example}}
```

### Load Configuration Settings into a Singleton {#load_configuration_into_a_singleton}

[![config][c~config~docs~badge]][c~config~docs] [![config~crates.io][c~config~crates.io~badge]][c~config~crates.io] [![config~repo][c~config~repo~badge]][c~config~repo] [![config~lib.rs][c~config~lib.rs~badge]][c~config~lib.rs]{{hi:config}}{{hi:Environment}}{{hi:Env}}{{hi:Settings}}{{hi:config}}{{hi:Configuration}} [![cat~config][cat~config~badge]][cat~config]{{hi:Configuration}}

```rust,editable
{{#include ../../../crates/cats/config/examples/configuration/config_singleton.rs:example}}
```

### Test Configuration-loading Methods {#test_configuration_loading_methods}

[![config][c~config~docs~badge]][c~config~docs] [![config~crates.io][c~config~crates.io~badge]][c~config~crates.io] [![config~repo][c~config~repo~badge]][c~config~repo] [![config~lib.rs][c~config~lib.rs~badge]][c~config~lib.rs]{{hi:config}}{{hi:Environment}}{{hi:Env}}{{hi:Settings}}{{hi:config}}{{hi:Configuration}} [![cat~config][cat~config~badge]][cat~config]{{hi:Configuration}}

```rust,editable
{{#include ../../../crates/cats/config/examples/configuration/config_testing.rs:example}}
```

## Load a TOML Configuration File with `confy` {#confy}

[![confy][c~confy~docs~badge]][c~confy~docs] [![confy~crates.io][c~confy~crates.io~badge]][c~confy~crates.io] [![confy~repo][c~confy~repo~badge]][c~confy~repo] [![confy~lib.rs][c~confy~lib.rs~badge]][c~confy~lib.rs]{{hi:confy}}

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
