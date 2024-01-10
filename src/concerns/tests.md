# Tests

`cargo test` to run all tests.
`cargo test test_prefix` to run all tests that start with the provided prefix.
`cargo test -- --show-output` to show output (println!) that is otherwise captured during tests.

```rust,editable
{{#include ../../deps/examples/tests.rs}}
```

## Custom message

```rust,editable,should_panic
{{#include ../../deps/examples/tests_custom_message.rs}}
```

## See Also

[![approx-badge]][approx] [Approx][approx]⮳

[cargo-nextest][cargo-nextest]⮳: `cargo nextest run; cargo test --doc`

{{#include ../refs/link-refs.md}}
