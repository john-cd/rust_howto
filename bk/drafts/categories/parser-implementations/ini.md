# INI Parsing

{{#include ini.incl.md}}

## Parse INI Configuration Files with `rust-ini` {#ini}

[![rust-ini][c-rust_ini-badge]][c-rust_ini] [![rust-ini-crates.io][c-rust_ini-crates.io-badge]][c-rust_ini-crates.io] [![rust-ini-github][c-rust_ini-github-badge]][c-rust_ini-github] [![rust-ini-lib.rs][c-rust_ini-lib.rs-badge]][c-rust_ini-lib.rs]{{hi:rust-ini}}{{hi:INI}}{{hi:Configuration}}

[`rust-ini`][c-rust_ini]⮳{{hi:rust-ini}} is an INI configuration file parsing library.

```rust,editable
{{#include ../../../crates/cats/parser_implementations/examples/ini/rust_ini.rs:example}}
```

## Other Options {#skip}

[`ini`][c-ini]⮳{{hi:ini}} can be used for parsing INI configuration files. [`ini`][c-ini]⮳{{hi:ini}} is a simple macro built on top of `configparser`.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[Write; Also consider: `configparser`](https://github.com/john-cd/rust_howto/issues/1185)
</div>
