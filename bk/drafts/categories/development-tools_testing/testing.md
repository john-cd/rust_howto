# Testing

{{#include testing.incl.md}}

For most basic code testing needs, using `cargo test` will be sufficient. For more advanced testing, especially property-based testing, [`proptest`][c-proptest]⮳{{hi:proptest}} is highly recommended. [`mockall`][c-mockall]⮳{{hi:mockall}} or [`faux`][c-faux]⮳{{hi:faux}} will help when you need to mock dependencies. [`rstest`][c-rstest]⮳{{hi:rstest}} is great for parameterized tests. And for ensuring your tests cover a good portion of your code, [`grcov`][c-grcov]⮳{{hi:grcov}} or [`tarpaulin`][c-tarpaulin]⮳{{hi:tarpaulin}} can be used.

## Unit Testing {#unit_testing}

`cargo test` (built-in) is the standard Rust testing framework for writing unit tests. Uses `#[test]` attribute.

## Integration Testing {#integration_testing}

Often uses `cargo test` as well, but integration tests are placed in a separate tests directory and focus on testing interactions between [modules][p-modules] or components.

[`rstest`][c-rstest]⮳{{hi:rstest}} is a resource-based test framework for writing data-driven tests.

This framework focuses on integration-testing, that means external software, not necessarily written in rust.
`rtest` works by using stateful resources. It uses [macros][p-macros] to build a executable binary that can handle all your filters and returns a nice output.

## Documentation Testing {#doc_testing}

Uses `#[doc = "```"]` in doc comments to embed testable examples in your documentation.

## Snapshot Testing: Test Your Code Against Snapshots {#insta}

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

Snapshots tests (also sometimes called approval tests) are tests that assert values against a reference value (the snapshot). Think of it as a supercharged version of [`assert_eq!`][c-std::assert_eq]⮳{{hi:assert_eq!}}. [`insta`][insta-website] lets you compare the result of a test against a reference value but, unlike simple assertions, the reference value is managed by [`insta`][c-insta]⮳{{hi:insta}} for you.

First, install the CLI with `cargo install cargo-insta`. Second, create a test, run it a first time with `cargo test`. This creates a snapshot file (ending with `.snap`). Use `cargo insta review` to review and accept the snapshot. Running `cargo test` again now succeeds, until the value returned by the function under test changes.

```rust,editable
{{#include ../../../crates/cats/development_tools_testing/tests/testing/insta.rs:example}}
```

## Asynchronous Testing {#async_testing}

Often involves using `tokio::test` or similar runtime-specific [attributes][p-attributes] for testing [asynchronous][p-asynchronous] code.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[testing: write](https://github.com/john-cd/rust_howto/issues/340)

- [serial_test](https://docs.rs/serial_test/latest/serial_test/) serial_test allows for the creation of serialised Rust tests using the serial attribute

</div>
