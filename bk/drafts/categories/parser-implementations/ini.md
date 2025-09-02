# INI Parsing

{{#include ini.incl.md}}

## Parse INI Configuration Files with `rust-ini` {#ini}

[![rust-ini][c~rust-ini~docs~badge]][c~rust-ini~docs] [![rust-ini~crates.io][c~rust-ini~crates.io~badge]][c~rust-ini~crates.io] [![rust-ini~repo][c~rust-ini~repo~badge]][c~rust-ini~repo] [![rust-ini~lib.rs][c~rust-ini~lib.rs~badge]][c~rust-ini~lib.rs]{{hi:rust-ini}}{{hi:INI}}{{hi:Configuration}}

[`rust-ini`][c~rust-ini~docs]↗{{hi:rust-ini}} is an INI configuration file parsing library.

```rust,editable
{{#include ../../../crates/cats/parser_implementations/examples/ini/rust-ini.rs:example}}
```

## Other Options {#other-options .skip}

[`ini`][c~ini~docs]↗{{hi:ini}} can be used for parsing INI configuration files. [`ini`][c~ini~docs]↗{{hi:ini}} is a simple macro built on top of `configparser`.

## Related Topics {#related-topics .skip}

FIXME

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[Write; Also consider: `configparser`](https://github.com/john-cd/rust_howto/issues/1185)
</div>
