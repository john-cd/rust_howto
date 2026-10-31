<!-- TODO Reviewed ??-??-2026 -->

# Mocking

{{#include mocking.incl.md}}

A test double replaces a dependency so a test can control a boundary and observe how the code under test uses it. A fake has a small working implementation, a stub returns prepared values, and a mock also records calls and checks configured expectations.

Prefer a fake or a real in-memory implementation when the test can assert the resulting state directly. Mocks are useful when an interaction is itself part of the contract, such as making sure a payment is sent once or that a failed write is handled. Mocking every internal call couples a test to implementation details and makes harmless refactoring harder.

In Rust, a trait often gives a service a narrow boundary that can be implemented by production code and a test double. Keep the interface focused on the behavior the caller needs.

Consider these libraries:

- [`mockall`][c~mockall~docs]↗{{hi:mockall}}: A popular crate for creating mock objects for testing.
- [`faux`][c~faux~docs]↗{{hi:faux}}: Another mocking library.
- [`httpmock`][c~httpmock~docs]↗{{hi:httpmock}}: A mock HTTP server for testing clients.

## Mock with `mockall` {#mocking}

{{#badges mockall development-tools::testing=Testing}}{{hi:Mock}}{{hi:Mocking}}{{hi:Testing}}

`mockall` can generate mocks for traits and set expectations for arguments, call counts, and return values. The example injects a repository into a service and verifies a persistence interaction as well as the returned behavior.

```rust,editable
{{#include ../../../crates/cats/development_tools_testing/examples/mocking/mockall.rs:example}}
```

## Mock Structs with `faux` {#faux}

{{#badges faux}}{{hi:faux}}{{hi:Mock}}{{hi:Mocking}}{{hi:Test}}{{hi:Testing}}

`faux` provides a way to mock methods on structs when introducing a trait boundary is not practical. Keep this choice local to tests: a trait is usually a clearer seam when callers should depend on behavior rather than a concrete type.

```rust,editable
{{#include ../../../crates/cats/development_tools_testing/examples/mocking/faux.rs:example}}
```

## Mock an HTTP Server with `httpmock` {#httpmock}

[![httpmock][c~httpmock~docs~badge]][c~httpmock~docs] [![httpmock~crates.io][c~httpmock~crates.io~badge]][c~httpmock~crates.io] [![httpmock~repo][c~httpmock~repo~badge]][c~httpmock~repo]{{hi:httpmock}}{{hi:Mock}}{{hi:Mocking}}{{hi:Testing}} [![cat~development-tools::testing][cat~development-tools::testing~badge]][cat~development-tools::testing]{{hi:Testing}}

Code that calls a web service can be tested without the network by pointing the client at a mock server on a free local port. [`httpmock`][c~httpmock~docs]↗{{hi:httpmock}} starts the server, lets each test describe the request it expects (`when`) and the response to send back (`then`), and checks that the request was received (`mock.assert()`). Because the client only needs a base URL, the same test works with any HTTP client. Use it for error paths that are hard to trigger on a real service, such as a `503` response. `wiremock` is an async-first alternative, and [`mockito`][c~mockito~docs]↗{{hi:mockito}} is a lighter one.

```rust,editable
{{#include ../../../crates/cats/development_tools_testing/examples/test_techniques/http_mock.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[testing | Testing]] describes unit, integration, and documentation tests.
- [[test_runners | Test Runners]] covers commands that execute the test suite.
- [[assertions | Assertions]] covers checks on results and behavior.
- [[property_based_testing | Property-based Testing]] is useful when a generated range of inputs can exercise a contract more thoroughly than a few mocked examples.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  testing: write
</div>
