<!-- TODO Reviewed ??-??-2026 -->

# Graph Theory

{{#include graph_theory.incl.md}}

## Schedule DAG Tasks by Level {#schedule-dag-tasks-by-level}

[![petgraph][c~petgraph~docs~badge]][c~petgraph~docs]{{hi:petgraph}} [![rayon][c~rayon~docs~badge]][c~rayon~docs]{{hi:rayon}} [![cat~science][cat~science~badge]][cat~science]

A directed acyclic graph (DAG) represents tasks and their prerequisites. Kahn's algorithm groups ready tasks into levels; tasks in each level can run in parallel because their prerequisites are complete. This example checks for cycles with `petgraph` before calculating the levels and uses `rayon` to evaluate each level.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/graph_theory/topological_levels.rs:example}}
```

## Find Cyclic Dependencies {#find-cyclic-dependencies}

[![petgraph][c~petgraph~docs~badge]][c~petgraph~docs]{{hi:petgraph}} [![cat~science][cat~science~badge]][cat~science]

`is_cyclic_directed` detects whether a directed graph contains a cycle. Tarjan's strongly connected components algorithm identifies the nodes that participate in cycles.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/graph_theory/cycle_detection.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[algorithms | Algorithms]].
- [[data_structures | Data Structures]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [accuracy review: mathematics](https://github.com/john-cd/rust_howto/issues/1894)
</div>
