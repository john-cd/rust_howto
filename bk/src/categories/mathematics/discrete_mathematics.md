<!-- TODO Reviewed ??-??-2026 -->

# Discrete Mathematics

{{#include discrete_mathematics.incl.md}}

## Compare Sets with `BTreeSet` {#compare-sets-with-btreeset}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~science][cat~science~badge]][cat~science]

The standard library's [`BTreeSet`][c~std::collections::BTreeSet~docs]↗{{hi:BTreeSet}} supports set operations and keeps values in order. This example finds shared prerequisites, all prerequisites, and values unique to one set.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/discrete_mathematics/set_operations.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[data_structures | Data Structures]].
- [[algorithms | Algorithms]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [accuracy review: mathematics](https://github.com/john-cd/rust_howto/issues/1894)
</div>
