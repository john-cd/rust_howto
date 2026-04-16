# Graphs

{{#include graphs.incl.md}}

## Create and Manipulate Graphs with `petgraph` {#petgraph}

[![petgraph][c~petgraph~docs~badge]][c~petgraph~docs] [![petgraph~crates.io][c~petgraph~crates.io~badge]][c~petgraph~crates.io] [![petgraph~repo][c~petgraph~repo~badge]][c~petgraph~repo] [![petgraph~lib.rs][c~petgraph~lib.rs~badge]][c~petgraph~lib.rs]{{hi:petgraph}}{{hi:Graph}}{{hi:Unionfind}}{{hi:Graph algorithms}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

`petgraph` is a graph data structure library. `petgraph` provides several graph types (each differing in the tradeoffs taken in their internal representation), algorithms on those graphs, and functionality to output graphs in 'graphviz' format. Both nodes and edges can have arbitrary associated data, and edges may be either directed or undirected.

The following example demonstrates creating a graph, adding nodes and edges, traversing the graph, and using different graph types:

```rust,editable,noplayground
{{#include ../../../crates/cats/data_structures/examples/graph/petgraph.rs:example}}
```

## See Also {#see-also .skip}

- [`pathfinding`][c~pathfinding~docs]↗{{hi:pathfinding}} provides pathfinding algorithms.

## Related Topics {#related-topics .skip}

- [[linkedlist | Linked Lists]].
- [[maps | Maps]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
