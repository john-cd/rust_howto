# Tests

Tools to help you verify the correctness{{hi:Correctness}} of your code.

{{#include index.incl.md}}

[![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}}   [![cat-testing][cat-testing-badge]][cat-testing]{{hi:Testing}}

`cargo test` to run all tests.
`cargo test test_prefix` to run all tests that start with the provided prefix.
`cargo test -- --show-output` to show output (println!) that is otherwise captured during tests.

```rust
{{#include ../../../deps/tests/tests.rs}}
```

## Custom message

[![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}}   [![cat-testing][cat-testing-badge]][cat-testing]{{hi:Testing}}

```rust
{{#include ../../../deps/tests/tests_custom_message.rs}}
```

## See Also

[![cat-testing][cat-testing-badge]][cat-testing]{{hi:Testing}}

[![approx][c-approx-badge]][c-approx]{{hi:approx}}  [`approx`][c-approx]{{hi:approx}}⮳

[cargo-nextest][c-cargo_nextest-website]⮳: `cargo nextest run; cargo test --doc`  [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO: expand: approx, nextest
</div>
