# Test Runners

{{#include test_runners.incl.md}}

## Test your code with `cargo test` {#cargo-test}

[![cargo][c~cargo~docs~badge]][c~cargo~docs]{{hi:cargo}}
[![cargo~crates.io][c~cargo~crates.io~badge]][c~cargo~crates.io]
[![cargo~github][c~cargo~github~badge]][c~cargo~github]
[![cargo~lib.rs][c~cargo~lib.rs~badge]][c~cargo~lib.rs]

[`cargo test`](https://doc.rust-lang.org/cargo/commands/cargo-test.html)↗{{hi:cargo test}} to run all tests.
`cargo test test_prefix` to run all tests that start with the provided prefix.
`cargo test -- --show-output` to show output (println!) that is otherwise captured during tests.

```rust,editable
{{#include ../../../crates/cats/development_tools_testing/examples/test_runners/tests1.rs:example}}
```

## Test your code Faster with `cargo nextest` {#cargo-nextest}

[![cargo-nextest][c~cargo_nextest~docs~badge]][c~cargo_nextest~docs]{{hi:cargo-nextest}}
[![cargo-nextest~crates.io][c~cargo_nextest~crates.io~badge]][c~cargo_nextest~crates.io]
[![cargo-nextest~github][c~cargo_nextest~github~badge]][c~cargo_nextest~github]
[![cargo-nextest~lib.rs][c~cargo_nextest~lib.rs~badge]][c~cargo_nextest~lib.rs]
[![cat~development-tools][cat~development-tools~badge]][cat~development-tools]

[`cargo-nextest`][c~cargo_nextest~website]↗ is a new, faster test runner for Rust.

```sh
cargo nextest run
cargo test --doc
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[testing: write](https://github.com/john-cd/rust_howto/issues/340)
</div>
