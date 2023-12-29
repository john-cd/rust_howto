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

[Approx]( https://docs.rs/approx/latest/approx/index.html )

[cargo-nextest]( https://nexte.st/ ): `cargo nextest run; cargo test --doc`
