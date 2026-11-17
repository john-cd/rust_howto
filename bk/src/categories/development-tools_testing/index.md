<!-- TODO Reviewed ??-??-2026. -->

# Development Tools - Testing

[![cat~development-tools::testing][cat~development-tools::testing~badge]][cat~development-tools::testing]{{hi:Development tools::Testing}}

Testing techniques and tools help find defects and protect behavior as code changes. Choose tests that exercise observable behavior at the level where a failure can be diagnosed, and keep a fast, repeatable test suite as the default feedback loop.

## Testing

{{#include testing.incl.md}}

### Snapshot Testing

{{#include snapshot_testing.incl.md}}

### Test Runners

Cargo's built-in test command is a good default. Alternative runners can improve parallelism, filtering, or output for larger workspaces.

{{#include test_runners.incl.md}}

### Assertion Libraries

{{#include assertions.incl.md}}

### Mocking

Use mocks to control or verify interactions at a boundary, such as a network or persistence interface. For ordinary domain logic, a small fake or a real in-memory implementation is often easier to understand.

{{#include mocking.incl.md}}

### Property-based Testing

{{#include property_based_testing.incl.md}}

## Code Coverage

Coverage shows which instrumented code ran for a particular test configuration. It can reveal untested paths, but it does not show whether assertions check the right behavior.

{{#include code_coverage.incl.md}}

## Mutation Testing

{{#include mutation_testing.incl.md}}

## Compiler Diagnostics

Use [`trybuild`][c~trybuild~repo]↗ to compile passing and failing fixture programs from an ordinary test. A failing fixture's expected compiler diagnostics are stored alongside it, so changes to macro errors or trait bounds can be reviewed like other snapshots.

## Fuzzing

{{#include fuzzing.incl.md}}

## Related Topics {#related-topics .skip}

- [Asynchronous Programming](../asynchronous/index.md) for testing asynchronous code.
- [Concurrency](../concurrency/index.md) for testing thread and task behavior.
- [Command-line Interface](../command-line-interface/index.md) for testing command-line programs.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
