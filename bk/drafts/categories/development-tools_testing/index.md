<!-- TODO Reviewed ??-??-2026 -->

# Development Tools - Tests

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

## Fuzzing

{{#include fuzzing.incl.md}}

## Related Topics {#related-topics .skip}

- [[testing | Testing]]
- [[snapshot_testing | Snapshot Testing]]
- [[code_coverage | Code Coverage]]
- [[fuzzing | Fuzzing]]
- [[property_based_testing | Property-based Testing]]
- [[mocking | Mocking]]
- [[test_runners | Test Runners]]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
expand

- [trybuild][c~trybuild~repo]↗: Test harness for ui tests of compiler diagnostics.

</div>
