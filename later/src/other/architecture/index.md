<!-- TODO Reviewed ??-??-2026. -->

# Architecture in Rust

{{#include architecture.incl.md}}{{hi:Architecture}}

Start with the architecture process to identify the system boundaries and
quality requirements. Use the pattern examples to compare approaches for
organizing business logic, integrations, and state changes.

## Plugin Registration

For components linked into the same executable, use an explicit registry
when the application needs to control registration order and handle errors.
When many modules need to register implementations independently, consider
[`inventory`][c~inventory~lib.rs]↗ for collecting registrations at link time.
Define a contract for duplicate registrations and ordering before dispatching
to a plugin. Run-time loading of external libraries requires a separate ABI
and versioning strategy.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
- [tangleguard~website][tangleguard~website]↗. (poor quality?)

</div>
