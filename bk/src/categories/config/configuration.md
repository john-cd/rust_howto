# Configuration

{{#include configuration.incl.md}}

## `config` {#config}

[![config][c-config-badge]][c-config] [![config-crates.io][c-config-crates.io-badge]][c-config-crates.io] [![config-github][c-config-github-badge]][c-config-github] [![config-lib.rs][c-config-lib.rs-badge]][c-config-lib.rs]{{hi:config}}{{hi:Environment}}{{hi:Env}}{{hi:Settings}}{{hi:config}}{{hi:Configuration}} [![cat-config][cat-config-badge]][cat-config]{{hi:Configuration}}

[`config`][c-config]{{hi:config}}⮳ is a layered configuration repository. It maintains a set of configuration sources,
fetches values to populate those, and provides them according to the source's priority.

Config lets you set a set of default parameters and then extend them via merging in configuration from a variety of sources:

- Files in [JSON][p-json], [TOML][p-toml], [[yaml | YAML]], [[ini | INI]], RON, [JSON][p-json]5; and
- Environment variables.
- String literals in well-known formats.
- Programmatic overrides.

```rust,editable
{{#include ../../../crates/cats/config/tests/config1.rs:example}}
```

```rust,editable
{{#include ../../../crates/cats/config/tests/config2.rs:example}}
```

```rust,editable
{{#include ../../../crates/cats/config/tests/config3.rs:example}}
```

## `confy` {#confy}

[![confy][c-confy-badge]][c-confy] [![confy-crates.io][c-confy-crates.io-badge]][c-confy-crates.io] [![confy-github][c-confy-github-badge]][c-confy-github] [![confy-lib.rs][c-confy-lib.rs-badge]][c-confy-lib.rs]{{hi:confy}}

[`confy`][c-confy]⮳{{hi:confy}} is a Rust crate that simplifies reading and writing TOML or YAML configuration files. It uses [`serde`][c-serde]⮳{{hi:serde}} for easy serialization/deserialization of config structs, handles file I/O, and supports defaults and environment variable overrides.

```rust,editable,noplayground
{{#include ../../../crates/cats/config/tests/confy.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[configuration: expand; add text](https://github.com/john-cd/rust_howto/issues/270)
</div>
