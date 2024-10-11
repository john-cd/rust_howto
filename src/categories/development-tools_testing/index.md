# Tests

Tools to help you verify the {{i:correctness}} of your code.

{{#include index.incl.md}}

[![cat-development-tools][cat-development-tools-badge]][cat-development-tools]   [![cat-testing][cat-testing-badge]][cat-testing]

`cargo test` to run all {{i:tests}}.
`cargo test test_prefix` to run all tests that start with the provided prefix.
`cargo test -- --show-output` to show output (println!) that is otherwise captured during tests.

```rust,editable
{{#include ../../../deps/tests/tests.rs}}
```

## Custom message

[![cat-development-tools][cat-development-tools-badge]][cat-development-tools]   [![cat-testing][cat-testing-badge]][cat-testing]

```rust,editable
{{#include ../../../deps/tests/tests_custom_message.rs}}
```

## See Also

[![cat-testing][cat-testing-badge]][cat-testing]

[![approx][c-approx-badge]][c-approx]  {{hi:Approx}}[`Approx`][c-approx]⮳

[cargo-nextest][c-cargo_nextest-website]⮳: `cargo nextest run; cargo test --doc`  [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
