# Mocking

{{#include mocking.incl.md}}

Consider using:

- [`mockall`][c-mockall]⮳{{hi:mockall}}: A popular crate for creating mock objects for testing.
- [`faux`][c-faux]⮳{{hi:faux}}: Another mocking library.

## Mock with `mockall` {#mocking}

[![mockall][c-mockall-badge]][c-mockall] [![mockall-crates.io][c-mockall-crates.io-badge]][c-mockall-crates.io] [![mockall-github][c-mockall-github-badge]][c-mockall-github] [![mockall-lib.rs][c-mockall-lib.rs-badge]][c-mockall-lib.rs]{{hi:mockall}}{{hi:Mock}}{{hi:Mocking}}{{hi:Testing}} [![cat-development-tools::testing][cat-development-tools::testing-badge]][cat-development-tools::testing]{{hi:Testing}}

A powerful mock object library for Rust.

```rust,editable
{{#include ../../../crates/cats/development_tools_testing/tests/mocking/mockall.rs:example}}
```

## Mock Structs with `faux` {#faux}

[![faux][c-faux-badge]][c-faux] [![faux-crates.io][c-faux-crates.io-badge]][c-faux-crates.io] [![faux-github][c-faux-github-badge]][c-faux-github] [![faux-lib.rs][c-faux-lib.rs-badge]][c-faux-lib.rs]{{hi:faux}}{{hi:faux}}{{hi:Mock}}{{hi:Mocking}}{{hi:Test}}{{hi:Testing}}

A library to mock structs.

```rust,editable
{{#include ../../../crates/cats/development_tools_testing/tests/mocking/faux.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[testing: write](https://github.com/john-cd/rust_howto/issues/340)
</div>
