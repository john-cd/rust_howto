<!-- TODO Reviewed ??-??-2026 -->

# Mocking

{{#include mocking.incl.md}}

A test double replaces a dependency so a test can control a boundary and observe how the code under test uses it. A fake has a small working implementation, a stub returns prepared values, and a mock also records calls and checks configured expectations.

Prefer a fake or a real in-memory implementation when the test can assert the resulting state directly. Mocks are useful when an interaction is itself part of the contract, such as making sure a payment is sent once or that a failed write is handled. Mocking every internal call couples a test to implementation details and makes harmless refactoring harder.

In Rust, a trait often gives a service a narrow boundary that can be implemented by production code and a test double. Keep the interface focused on the behavior the caller needs.

Consider these libraries:

- [`mockall`][c~mockall~docs]↗{{hi:mockall}}: A popular crate for creating mock objects for testing.
- [`faux`][c~faux~docs]↗{{hi:faux}}: Another mocking library.

## Mock with `mockall` {#mocking}

[![mockall][c~mockall~docs~badge]][c~mockall~docs] [![mockall~crates.io][c~mockall~crates.io~badge]][c~mockall~crates.io] [![mockall~repo][c~mockall~repo~badge]][c~mockall~repo] [![mockall~lib.rs][c~mockall~lib.rs~badge]][c~mockall~lib.rs]{{hi:mockall}}{{hi:Mock}}{{hi:Mocking}}{{hi:Testing}} [![cat~development-tools::testing][cat~development-tools::testing~badge]][cat~development-tools::testing]{{hi:Testing}}

`mockall` can generate mocks for traits and set expectations for arguments, call counts, and return values. The example injects a repository into a service and verifies a persistence interaction as well as the returned behavior.

```rust,editable
{{#include ../../../crates/cats/development_tools_testing/examples/mocking/mockall.rs:example}}
```

## Mock Structs with `faux` {#faux}

[![faux][c~faux~docs~badge]][c~faux~docs] [![faux~crates.io][c~faux~crates.io~badge]][c~faux~crates.io] [![faux~repo][c~faux~repo~badge]][c~faux~repo] [![faux~lib.rs][c~faux~lib.rs~badge]][c~faux~lib.rs]{{hi:faux}}{{hi:faux}}{{hi:Mock}}{{hi:Mocking}}{{hi:Test}}{{hi:Testing}}

`faux` provides a way to mock methods on structs when introducing a trait boundary is not practical. Keep this choice local to tests: a trait is usually a clearer seam when callers should depend on behavior rather than a concrete type.

```rust,editable
{{#include ../../../crates/cats/development_tools_testing/examples/mocking/faux.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[testing | Testing]] describes unit, integration, and documentation tests.
- [[test_runners | Test Runners]] covers commands that execute the test suite.
- [[assertions | Assertions]] covers checks on results and behavior.
- [[property_based_testing | Property-based Testing]] is useful when a generated range of inputs can exercise a contract more thoroughly than a few mocked examples.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[testing: write](https://github.com/john-cd/rust_howto/issues/340)
</div>
