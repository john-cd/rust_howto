# Assertion Libraries

{{#include assertions.incl.md}}

## Emit a Custom Message {#custom-message}

[![std][c~std~docs~badge]][c~std~docs] [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}} [![cat~development-tools::testing][cat~development-tools::testing~badge]][cat~development-tools::testing]{{hi:Testing}}

```rust,editable
{{#include ../../../crates/cats/development_tools_testing/examples/assertions/tests_custom_message.rs:example}}
```

## Assert that a Value Matches a Pattern with `assert_matches` {#assertion_libraries}

[![assert_matches][c~assert_matches~docs~badge]][c~assert_matches~docs] [![assert_matches~crates.io][c~assert_matches~crates.io~badge]][c~assert_matches~crates.io] [![assert_matches~github][c~assert_matches~github~badge]][c~assert_matches~github] [![assert_matches~lib.rs][c~assert_matches~lib.rs~badge]][c~assert_matches~lib.rs]{{hi:assert_matches}}{{hi:Pattern}}{{hi:Assert}}{{hi:Match}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

[`assert_matches`][c~assert_matches~docs]↗{{hi:assert_matches}}: A crate for matching on patterns in assertions.

## `approx` {#approx}

[![approx][c~approx~docs~badge]][c~approx~docs]{{hi:approx}}
[![approx~crates.io][c~approx~crates.io~badge]][c~approx~crates.io]
[![approx~github][c~approx~github~badge]][c~approx~github]
[![approx~lib.rs][c~approx~lib.rs~badge]][c~approx~lib.rs]
[![cat~development-tools::testing][cat~development-tools::testing~badge]][cat~development-tools::testing]{{hi:Testing}}

[`approx`][c~approx~docs]{{hi:approx}}↗ allows approximate floating point equality comparisons and assertions.

```rust,editable
{{#include ../../../crates/cats/development_tools_testing/examples/assertions/approx.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1174)

- [pretty_assertions — Rust dev tool][c~pretty_assertions~lib.rs]

</div>
