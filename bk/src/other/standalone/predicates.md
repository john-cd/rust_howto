<!-- TODO Reviewed ??-??-2026. -->

# Predicates

A predicate is a function or object that answers a yes-or-no question about a value. The `predicates` crate provides reusable boolean checks and combinators for assembling more complex conditions.

## Compose Predicates to Filter Values {#composite-predicate}

The example adapts a substring check and a numeric comparison to fields on a file record, combines them with `and`, and filters records at runtime. Its expected result is asserted before it is printed.

[![predicates][c~predicates~docs~badge]][c~predicates~docs]↗

```rust,noplayground
{{#include ../../../crates/cats/development_tools_testing/examples/predicates/composite_predicate.rs:example}}
```

The same predicate building blocks can support validation or assertions, but keep error reporting at the boundary that knows how to explain a failed check. For tests that assert command output, see [Assertions](../../categories/development-tools_testing/assertions.md).

## Related Topics {#related-topics .skip}

- [Assertions](../../categories/development-tools_testing/assertions.md).
- [String Search and Fuzzy Matching](../../categories/text-processing/string_search.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
