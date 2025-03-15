# INI Parsing

{{#include ini.incl.md}}

## Parse INI Configuration Files with `rust-ini` {#ini}

[![rust-ini][c-rust_ini-badge]][c-rust_ini] [![rust-ini-crates.io][c-rust_ini-crates.io-badge]][c-rust_ini-crates.io] [![rust-ini-github][c-rust_ini-github-badge]][c-rust_ini-github] [![rust-ini-lib.rs][c-rust_ini-lib.rs-badge]][c-rust_ini-lib.rs]{{hi:rust-ini}}{{hi:INI}}{{hi:Configuration}}

[`rust-ini`][c-rust_ini]⮳{{hi:rust-ini}} is an INI configuration file parsing library.

```rust,editable
{{#include ../../../crates/cats/parser_implementations/tests/ini/rust_ini.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO Write

Also consider:

`configparser`

[![ini][c-ini-badge]][c-ini] [![ini-crates.io][c-ini-crates.io-badge]][c-ini-crates.io] [![ini-github][c-ini-github-badge]][c-ini-github] [![ini-lib.rs][c-ini-lib.rs-badge]][c-ini-lib.rs]{{hi:ini}}{{hi:Macro}}{{hi:Settings}}{{hi:Config}}{{hi:ini}}{{hi:Configuration}} [![cat-parser-implementations][cat-parser-implementations-badge]][cat-parser-implementations]{{hi:Parser implementations}} [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}} [![cat-config][cat-config-badge]][cat-config]{{hi:Configuration}}

A simple macro built on top of configparser to load and parse ini files. You can use this to write Rust programs which can be customized by end users easily. [`ini`][c-ini]⮳{{hi:ini}} can be used for parsing INI configuration files.

</div>
