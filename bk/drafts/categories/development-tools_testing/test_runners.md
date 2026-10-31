<!-- TODO Reviewed ??-??-2026 -->

# Test Runners

{{#include test_runners.incl.md}}

## Test your code with `cargo test` {#cargo-test}

{{#badges cargo}}

[`cargo test`][book~cargo~cargo-test]↗{{hi:cargo test}} to run all tests.
`cargo test test_prefix` to run all tests that start with the provided prefix.
`cargo test -- --show-output` to show output (println!) that is otherwise captured during tests.

```rust,editable
{{#include ../../../crates/cats/development_tools_testing/examples/test_runners/tests1.rs:example}}
```

## Test your code Faster with `cargo nextest` {#cargo-nextest}

{{#badges cargo-nextest development-tools=}}

[`cargo-nextest`][c~cargo-nextest~website]↗ is a new, faster test runner for Rust.

```sh
cargo nextest run
cargo test --doc
```

## Related Topics {#related-topics .skip}

- [[testing | Testing]].
- [[code_coverage | Code Coverage]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  testing: write
</div>
