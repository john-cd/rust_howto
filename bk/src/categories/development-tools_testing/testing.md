# Testing

{{#include testing.incl.md}}

## Test your code with `cargo test` {#cargo-test}

[![cargo][c-cargo-badge]][c-cargo]{{hi:cargo}}
[![cargo-crates.io][c-cargo-crates.io-badge]][c-cargo-crates.io]
[![cargo-github][c-cargo-github-badge]][c-cargo-github]
[![cargo-lib.rs][c-cargo-lib.rs-badge]][c-cargo-lib.rs]

`cargo test` to run all tests.
`cargo test test_prefix` to run all tests that start with the provided prefix.
`cargo test -- --show-output` to show output (println!) that is otherwise captured during tests.

```rust,editable
{{#include ../../../crates/cats/development_tools_testing/tests/tests1.rs:example}}
```

## Emit a custom message {#custom-message}

[![std][c-std-badge]][c-std] [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}}  [![cat-development-tools::testing][cat-development-tools::testing-badge]][cat-development-tools::testing]{{hi:Testing}}

```rust,editable
{{#include ../../../crates/cats/development_tools_testing/tests/tests_custom_message.rs:example}}
```

## Test your code faster with `cargo nextest` {#cargo-nextest}

[![cargo-nextest][c-cargo_nextest-badge]][c-cargo_nextest]{{hi:cargo-nextest}}
[![cargo-nextest-crates.io][c-cargo_nextest-crates.io-badge]][c-cargo_nextest-crates.io]
[![cargo-nextest-github][c-cargo_nextest-github-badge]][c-cargo_nextest-github]
[![cargo-nextest-lib.rs][c-cargo_nextest-lib.rs-badge]][c-cargo_nextest-lib.rs]
[![cat-development-tools][cat-development-tools-badge]][cat-development-tools]

[`cargo-nextest`][c-cargo_nextest-website]⮳ is a new, faster test runner for Rust.

```sh
cargo nextest run
cargo test --doc
```

## `approx` {#approx}

[![approx][c-approx-badge]][c-approx]{{hi:approx}}
[![approx-crates.io][c-approx-crates.io-badge]][c-approx-crates.io]
[![approx-github][c-approx-github-badge]][c-approx-github]
[![approx-lib.rs][c-approx-lib.rs-badge]][c-approx-lib.rs]
[![cat-development-tools::testing][cat-development-tools::testing-badge]][cat-development-tools::testing]{{hi:Testing}}

[`approx`][c-approx]{{hi:approx}}⮳ allows approximate floating point equality comparisons and assertions.

```rust,editable
{{#include ../../../crates/cats/development_tools_testing/tests/approx.rs:example}}
```

## Test your code against snapshots {#insta}

[![insta][c-insta-badge]][c-insta]{{hi:insta}}
[![insta-crates.io][c-insta-crates.io-badge]][c-insta-crates.io]
[![insta-github][c-insta-github-badge]][c-insta-github]
[![insta-lib.rs][c-insta-lib.rs-badge]][c-insta-lib.rs]
[![cat-development-tools::testing][cat-development-tools::testing-badge]][cat-development-tools::testing]{{hi:Testing}}

[![cargo-insta][c-cargo_insta-badge]][c-cargo_insta]
[![cargo-insta-crates.io][c-cargo_insta-crates.io-badge]][c-cargo_insta-crates.io]
[![cargo-insta-github][c-cargo_insta-github-badge]][c-cargo_insta-github]
[![cargo-insta-lib.rs][c-cargo_insta-lib.rs-badge]][c-cargo_insta-lib.rs]
[![cat-development-tools::cargo-plugins][cat-development-tools::cargo-plugins-badge]][cat-development-tools::cargo-plugins]{{hi:Cargo plugins}}

Snapshots tests (also sometimes called approval tests) are tests that assert values against a reference value (the snapshot). Think of it as a supercharged version of [`assert_eq!`][assert_eq!]⮳{{hi:assert_eq!}}. [`insta`][insta-website] lets you compare the result of a test against a reference value but, unlike simple assertions, the reference value is managed by `insta` for you.

First, install the CLI with `cargo install cargo-insta`. Second, create a test, run it a first time with `cargo test`. This creates a snapshot file (ending with `.snap`). Use `cargo insta review` to review and accept the snapshot. Running `cargo test` again now succeeds, until the value returned by the function under test changes.

```rust,editable
{{#include ../../../crates/cats/development_tools_testing/tests/insta.rs:example}}
```

## Measure your code coverage {#cargo-tarpaulin}

[![cargo_tarpaulin-github][c-cargo_tarpaulin-github-badge]][c-cargo_tarpaulin-github]{{hi:cargo-tarpaulin}} [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}} [![cat-development-tools::cargo-plugins][cat-development-tools::cargo-plugins-badge]][cat-development-tools::cargo-plugins]{{hi:Cargo plugins}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[testing: write (P2)](https://github.com/john-cd/rust_howto/issues/340)

</div>
