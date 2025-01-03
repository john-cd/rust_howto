# Configuration

{{#include configuration.incl.md}}

## `config` {#config}

[![config][c-config-badge]][c-config] [![config-crates.io][c-config-crates.io-badge]][c-config-crates.io] [![config-github][c-config-github-badge]][c-config-github] [![config-lib.rs][c-config-lib.rs-badge]][c-config-lib.rs]{{hi:config}}{{hi:Environment}}{{hi:Env}}{{hi:Settings}}{{hi:config}}{{hi:Configuration}} [![cat-config][cat-config-badge]][cat-config]{{hi:Configuration}}

[`config`][c-config]{{hi:config}}⮳ is a layered configuration{{hi:Configuration}} system for Rust applications.

Config lets you set a set of default parameters and then extend them via merging in configuration from a variety of sources:
environment variables; string literals in well-known formats; files in JSON, TOML, YAML, INI, RON, JSON5; and programmatic overrides.

```rust,editable
{{#include ../../../crates/ex/categories/c/tests/config/config1.rs:example}}
```

## `confy` {#confy}

[![confy][c-confy-badge]][c-confy] [![confy-crates.io][c-confy-crates.io-badge]][c-confy-crates.io] [![confy-github][c-confy-github-badge]][c-confy-github] [![confy-lib.rs][c-confy-lib.rs-badge]][c-confy-lib.rs]{{hi:confy}}

```rust,editable,noplayground
{{#include ../../../crates/ex/categories/c/tests/config/confy.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[configuration: expand; add text (P1)](https://github.com/john-cd/rust_howto/issues/270)
</div>
