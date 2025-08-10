# Testing

{{#include testing.incl.md}}

For most basic code testing needs, using [`cargo test`](https://doc.rust-lang.org/cargo/commands/cargo-test.html)↗{{hi:cargo test}} will be sufficient. For more advanced testing, especially property-based testing, [`proptest`][c~proptest~docs]↗{{hi:proptest}} is highly recommended. [`mockall`][c~mockall~docs]↗{{hi:mockall}} or [`faux`][c~faux~docs]↗{{hi:faux}} will help when you need to mock dependencies. [`rstest`][c~rstest~docs]↗{{hi:rstest}} is great for parameterized tests. And for ensuring your tests cover a good portion of your code, [`grcov`][c~grcov~docs]↗{{hi:grcov}} or [`tarpaulin`][c~cargo-tarpaulin~docs]↗{{hi:tarpaulin}} can be used.

## Unit Testing {#unit_testing}

[`cargo test`](https://doc.rust-lang.org/cargo/commands/cargo-test.html)↗{{hi:cargo test}} (built-in) is the standard Rust testing framework for writing unit tests. Uses `#[test]` attribute.

## Integration Testing {#integration_testing}

Often uses [`cargo test`](https://doc.rust-lang.org/cargo/commands/cargo-test.html)↗{{hi:cargo test}} as well, but integration tests are placed in a separate tests directory and focus on testing interactions between [modules][p~modules] or components.

[`rstest`][c~rstest~docs]↗{{hi:rstest}} is a resource-based test framework for writing data-driven tests.

This framework focuses on integration-testing, that means external software, not necessarily written in rust.
`rtest` works by using stateful resources. It uses [macros][p~macros] to build a executable binary that can handle all your filters and returns a nice output.

## Documentation Testing {#doc_testing}

Uses `#[doc = "```"]` in doc comments to embed testable examples in your documentation.

## Snapshot Testing: Test Your Code Against Snapshots {#insta}

[![insta][c~insta~docs~badge]][c~insta~docs]{{hi:insta}}
[![insta~crates.io][c~insta~crates.io~badge]][c~insta~crates.io]
[![insta~github][c~insta~github~badge]][c~insta~github]
[![insta~lib.rs][c~insta~lib.rs~badge]][c~insta~lib.rs]
[![cat~development-tools::testing][cat~development-tools::testing~badge]][cat~development-tools::testing]{{hi:Testing}}

[![cargo-insta][c~cargo-insta~docs~badge]][c~cargo-insta~docs]
[![cargo-insta~crates.io][c~cargo-insta~crates.io~badge]][c~cargo-insta~crates.io]
[![cargo-insta~github][c~cargo-insta~github~badge]][c~cargo-insta~github]
[![cargo-insta~lib.rs][c~cargo-insta~lib.rs~badge]][c~cargo-insta~lib.rs]
[![cat~development-tools::cargo-plugins][cat~development-tools::cargo-plugins~badge]][cat~development-tools::cargo-plugins]{{hi:Cargo plugins}}

Snapshots tests (also sometimes called approval tests) are tests that assert values against a reference value (the snapshot). Think of it as a supercharged version of [`assert_eq!`][c~std::assert_eq~docs]↗{{hi:assert_eq!}}. [`insta`][insta~website] lets you compare the result of a test against a reference value but, unlike simple assertions, the reference value is managed by [`insta`][c~insta~docs]↗{{hi:insta}} for you.

First, install the CLI with `cargo install cargo-insta`. Second, create a test, run it a first time with [`cargo test`](https://doc.rust-lang.org/cargo/commands/cargo-test.html)↗{{hi:cargo test}}. This creates a snapshot file (ending with `.snap`). Use `cargo insta review` to review and accept the snapshot. Running `cargo test` again now succeeds, until the value returned by the function under test changes.

```rust,editable
{{#include ../../../crates/cats/development_tools_testing/examples/testing/insta.rs:example}}
```

## Asynchronous Testing {#async_testing}

Often involves using `tokio::test` or similar runtime-specific [attributes][p~attributes] for testing [asynchronous][p~asynchronous] code.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[testing: write](https://github.com/john-cd/rust_howto/issues/340)

- [serial_test](https://docs.rs/serial_test/latest/serial_test/) serial_test allows for the creation of serialized Rust tests using the serial attribute

</div>
