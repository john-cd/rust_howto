# Tests

Tools to help you verify the correctness of your code.

`cargo test` to run all tests.
`cargo test test_prefix` to run all tests that start with the provided prefix.
`cargo test -- --show-output` to show output (println!) that is otherwise captured during tests.

```rust,editable
{{#include ../../deps/examples/tests.rs}}
```

## Custom message

```rust,editable
{{#include ../../deps/examples/tests_custom_message.rs}}
```

## See Also

[![approx-badge]][approx] [`Approx`][Approx]⮳

[cargo-nextest][cargo-nextest-website]⮳: `cargo nextest run; cargo test --doc`

{{#include ../refs/link-refs.md}}
