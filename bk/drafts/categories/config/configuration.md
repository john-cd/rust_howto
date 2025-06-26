# Configuration

{{#include configuration.incl.md}}

## Load Application Settings from Multiple Sources with `config` {#config}

[![config][c~config~docs~badge]][c~config~docs] [![config~crates.io][c~config~crates.io~badge]][c~config~crates.io] [![config~github][c~config~github~badge]][c~config~github] [![config~lib.rs][c~config~lib.rs~badge]][c~config~lib.rs]{{hi:config}}{{hi:Environment}}{{hi:Env}}{{hi:Settings}}{{hi:config}}{{hi:Configuration}} [![cat~config][cat~config~badge]][cat~config]{{hi:Configuration}}

[`config`][c~config~docs]{{hi:config}}⮳ is a layered configuration repository. It lets you set a set of default parameters and then extend them via merging in configuration from a variety of sources:

- Files in [JSON][p~json], [TOML][p~toml], [[yaml | YAML]], [[ini | INI]], RON, [JSON][p~json]5; and
- Environment variables.
- String literals in well-known formats.
- Programmatic overrides.

```rust,editable
{{#include ../../../crates/cats/config/examples/configuration/config_hierarchical.rs:example}}
```

### Load Configuration Settings into a Singleton {#load_configuration_into_a_singleton}

[![config][c~config~docs~badge]][c~config~docs] [![config~crates.io][c~config~crates.io~badge]][c~config~crates.io] [![config~github][c~config~github~badge]][c~config~github] [![config~lib.rs][c~config~lib.rs~badge]][c~config~lib.rs]{{hi:config}}{{hi:Environment}}{{hi:Env}}{{hi:Settings}}{{hi:config}}{{hi:Configuration}} [![cat~config][cat~config~badge]][cat~config]{{hi:Configuration}}

```rust,editable
{{#include ../../../crates/cats/config/examples/configuration/config_singleton.rs:example}}
```

### Test Configuration-loading Methods {#test_configuration_loading_methods}

[![config][c~config~docs~badge]][c~config~docs] [![config~crates.io][c~config~crates.io~badge]][c~config~crates.io] [![config~github][c~config~github~badge]][c~config~github] [![config~lib.rs][c~config~lib.rs~badge]][c~config~lib.rs]{{hi:config}}{{hi:Environment}}{{hi:Env}}{{hi:Settings}}{{hi:config}}{{hi:Configuration}} [![cat~config][cat~config~badge]][cat~config]{{hi:Configuration}}

```rust,editable
{{#include ../../../crates/cats/config/examples/configuration/config_testing.rs:example}}
```

## Load Configuration from TOML or YAML with `confy` {#confy}

[![confy][c~confy~docs~badge]][c~confy~docs] [![confy~crates.io][c~confy~crates.io~badge]][c~confy~crates.io] [![confy~github][c~confy~github~badge]][c~confy~github] [![confy~lib.rs][c~confy~lib.rs~badge]][c~confy~lib.rs]{{hi:confy}}

[`confy`][c~confy~docs]⮳{{hi:confy}} is a Rust crate that simplifies reading and writing TOML or YAML configuration files. It uses [`serde`][c~serde~docs]⮳{{hi:serde}} for easy serialization/deserialization of config structs, handles file I/O, and supports defaults and environment variable overrides.

```rust,editable,noplayground
{{#include ../../../crates/cats/config/examples/configuration/confy.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[finish](https://github.com/john-cd/rust_howto/issues/270)
</div>
