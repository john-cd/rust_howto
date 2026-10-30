<!-- TODO Reviewed ??-??-2026 -->

# Testing

{{#include testing.incl.md}}

For most basic code testing needs, using [`cargo test`][book~cargo~cargo-test]↗{{hi:cargo test}} will be sufficient. For more advanced testing, especially property-based testing, [`proptest`][c~proptest~docs]↗{{hi:proptest}} is highly recommended. [`mockall`][c~mockall~docs]↗{{hi:mockall}} or [`faux`][c~faux~docs]↗{{hi:faux}} will mock dependencies. [`rstest`][c~rstest~docs]↗{{hi:rstest}} is great for parameterized tests. And for ensuring your tests cover a good portion of your code, [`grcov`][c~grcov~docs]↗{{hi:grcov}} or [`tarpaulin`][c~cargo-tarpaulin~docs]↗{{hi:tarpaulin}} can be used.

## Unit Testing {#unit_testing}

[`cargo test`][book~cargo~cargo-test]↗{{hi:cargo test}} (built-in) is the standard Rust testing framework for writing unit tests. Uses `#[test]` attribute.

## Compare Floating-Point Values Approximately {#float-asserts}

[![float-cmp][c~float-cmp~docs~badge]][c~float-cmp~docs] [![float-cmp~crates.io][c~float-cmp~crates.io~badge]][c~float-cmp~crates.io]{{hi:float-cmp}} [![approx][c~approx~docs~badge]][c~approx~docs]{{hi:approx}} [![cat~development-tools::testing][cat~development-tools::testing~badge]][cat~development-tools::testing]{{hi:Floating point}}

Floating-point values are rounded to representable binary numbers, so `0.1 + 0.2` is not exactly `0.3`. Avoid `assert_eq!` for computed floating-point results. [`float-cmp`][c~float-cmp~docs]↗{{hi:float-cmp}} compares by ULPs (units in the last place) or an absolute epsilon; [`approx`][c~approx~docs]↗{{hi:approx}} provides assertion macros such as `assert_relative_eq!`. Choose a tolerance that matches the scale and error bounds of the calculation.

```rust,editable
{{#include ../../../crates/cats/development_tools_testing/examples/test_techniques/float_asserts.rs:example}}
```

## Integration Testing {#integration_testing}

Often uses [`cargo test`][book~cargo~cargo-test]↗{{hi:cargo test}} as well, but integration tests are placed in a separate tests directory and focus on testing interactions between [modules][p~modules] or components.

[`rstest`][c~rstest~docs]↗{{hi:rstest}} is a resource-based test framework for writing data-driven tests.

This framework focuses on integration-testing, that means external software, not necessarily written in rust.
`rtest` works by using stateful resources. It uses [macros][p~macros] to build a executable binary that can handle all your filters and returns a nice output.

## Documentation Testing {#doc_testing}

Uses `#[doc = "```"]` in doc comments to embed testable examples in your documentation.

## Snapshot Testing: Test Your Code Against Snapshots {#insta}

[![insta][c~insta~docs~badge]][c~insta~docs]{{hi:insta}}
[![insta~crates.io][c~insta~crates.io~badge]][c~insta~crates.io]
[![insta~repo][c~insta~repo~badge]][c~insta~repo]
[![insta~lib.rs][c~insta~lib.rs~badge]][c~insta~lib.rs]
[![cat~development-tools::testing][cat~development-tools::testing~badge]][cat~development-tools::testing]{{hi:Testing}}

[![cargo-insta][c~cargo-insta~docs~badge]][c~cargo-insta~docs]
[![cargo-insta~crates.io][c~cargo-insta~crates.io~badge]][c~cargo-insta~crates.io]
[![cargo-insta~repo][c~cargo-insta~repo~badge]][c~cargo-insta~repo]
[![cargo-insta~lib.rs][c~cargo-insta~lib.rs~badge]][c~cargo-insta~lib.rs]
[![cat~development-tools::cargo-plugins][cat~development-tools::cargo-plugins~badge]][cat~development-tools::cargo-plugins]{{hi:Cargo plugins}}

Snapshots tests (also sometimes called approval tests) are tests that assert values against a reference value (the snapshot). Think of it as a supercharged version of [`assert_eq!`][c~std::assert_eq~docs]↗{{hi:assert_eq!}}. [`insta`][insta~website]↗ compares the result of a test against a reference value but, unlike simple assertions, the reference value is managed by [`insta`][c~insta~docs]↗{{hi:insta}} for you.

First, install the CLI with `cargo install cargo-insta`. Second, create a test, run it a first time with [`cargo test`][book~cargo~cargo-test]↗{{hi:cargo test}}. This creates a snapshot file (ending with `.snap`). Use `cargo insta review` to review and accept the snapshot. Running `cargo test` again now succeeds, until the value returned by the function under test changes.

```rust,editable
{{#include ../../../crates/cats/development_tools_testing/examples/testing/insta.rs:example}}
```

## Asynchronous Testing {#async_testing}

Often involves using [`tokio::test`][c~tokio::test~docs]↗{{hi:tokio::test}} or similar runtime-specific [attributes][p~attributes] for testing [asynchronous][p~asynchronous] code.

## Parameterized Tests and Fixtures (`rstest`) {#rstest}

[![rstest][c~rstest~docs~badge]][c~rstest~docs] [![rstest~crates.io][c~rstest~crates.io~badge]][c~rstest~crates.io]{{hi:rstest}} [![cat~development-tools::testing][cat~development-tools::testing~badge]][cat~development-tools::testing]{{hi:Parameterized tests}}

[`rstest`][c~rstest~docs]↗{{hi:rstest}} expands one test function into many: `#[case]` rows, `#[values]` combinations and `#[fixture]` functions for shared setup. Each expanded test has its own name, so a failure points at the failing row.

```rust,editable
{{#include ../../../crates/cats/development_tools_testing/examples/test_techniques/rstest_cases.rs:example}}
```

## Test Code that Reads Environment Variables {#env-vars}

[![cat~development-tools::testing][cat~development-tools::testing~badge]][cat~development-tools::testing]{{hi:Environment variables}}

Parse configuration from an injected value in unit tests. For integration tests of process environment behavior, set or remove variables on a child `std::process::Command`; this isolates the values from the test runner. A `serial_test` lock coordinates only tests that use the same lock, and a restoring wrapper does not protect against unrelated threads, libraries, or DNS code reading the process environment. In edition 2024, `std::env::set_var` and `remove_var` are unsafe on every target, but the safety precondition differs: on Windows they are sound even in multithreaded programs; on Unix they must not be called in a multithreaded program where other threads may read or write the environment.

```rust,editable
{{#include ../../../crates/cats/development_tools_testing/examples/test_techniques/env_vars.rs:example}}
```

## Test Timeouts Instantly with Paused Time {#paused-time}

[![tokio][c~tokio~docs~badge]][c~tokio~docs] [![tokio~crates.io][c~tokio~crates.io~badge]][c~tokio~crates.io]{{hi:tokio}} [![cat~development-tools::testing][cat~development-tools::testing~badge]][cat~development-tools::testing]{{hi:Async testing}}

With `#[tokio::test(start_paused = true)]` (the `test-util` feature of [`tokio`][c~tokio~docs]↗{{hi:tokio}}), the clock only advances when every task is idle, so timeouts and retry back-offs run instantly and deterministically.

```rust,editable
{{#include ../../../crates/cats/development_tools_testing/examples/test_techniques/paused_time.rs:example}}
```

## Generate Valid Test Inputs with a Custom `proptest` Strategy {#proptest-strategy}

[![proptest][c~proptest~docs~badge]][c~proptest~docs]{{hi:proptest}} [![cat~development-tools::testing][cat~development-tools::testing~badge]][cat~development-tools::testing]

Random inputs that violate a type's invariants waste test cases. [`prop_compose!`][c~proptest~docs]↗{{hi:proptest}} builds a strategy from smaller ones, and the second parameter list may depend on the first, so every generated `Span` satisfies `start <= end` by construction. Use `prop_assume!` to discard the remaining unwanted cases.

```rust,editable
{{#include ../../../crates/cats/development_tools_testing/examples/test_techniques/proptest_strategy.rs:example}}
```

## Test a Command-line Program with `assert_cmd` {#cli-testing}

[![assert_cmd][c~assert_cmd~docs~badge]][c~assert_cmd~docs]{{hi:assert_cmd}} [![assert_cmd~crates.io][c~assert_cmd~crates.io~badge]][c~assert_cmd~crates.io] [![assert_cmd~repo][c~assert_cmd~repo~badge]][c~assert_cmd~repo] [![assert_cmd~lib.rs][c~assert_cmd~lib.rs~badge]][c~assert_cmd~lib.rs] [![predicates][c~predicates~docs~badge]][c~predicates~docs]{{hi:predicates}} [![cat~development-tools::testing][cat~development-tools::testing~badge]][cat~development-tools::testing]

[`assert_cmd`][c~assert_cmd~docs]↗{{hi:assert_cmd}} runs a program and asserts on its exit status, `stdout` and `stderr`, using the [`predicates`][c~predicates~docs]↗ crate for matching. In an integration test (`tests/*.rs`) of a package that builds a binary, `Command::cargo_bin("cli-demo")` finds the executable built for the test. The example below defines a small CLI, then checks its help, success, and error behavior. To compare whole outputs, consider `snapbox` or `trycmd`, which keep expected output in snapshot files.

```rust,editable,noplayground
{{#include ../../../crates/cats/development_tools_testing/examples/cli_testing/cli_demo.rs:example}}
```

```rust,editable,noplayground
{{#include ../../../crates/cats/development_tools_testing/tests/cli_test.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[loom | Model-Checking Concurrent Code with Loom]] explores thread interleavings that ordinary tests miss.

FIXME

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
testing: write

- [serial_test][c~serial_test~docs]↗ allows for the creation of serialized Rust tests using the serial attribute.

</div>
