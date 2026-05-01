# Graph Algorithms

{{#include graph_algorithms.incl.md}}

Graph algorithms operate on data structures consisting of nodes (vertices) connected by edges. They are used in a wide variety of applications, including network routing, social network analysis, dependency resolution, and pathfinding in games.

The [`petgraph`][c~petgraph~docs]↗{{hi:petgraph}} crate provides graph data structures and a rich set of graph algorithms. It supports directed and undirected graphs, and provides implementations of many classic algorithms.

## Traverse a Graph with Breadth-First Search (BFS) {#petgraph-bfs}

[![petgraph][c~petgraph~docs~badge]][c~petgraph~docs] [![petgraph~crates.io][c~petgraph~crates.io~badge]][c~petgraph~crates.io] [![petgraph~repo][c~petgraph~repo~badge]][c~petgraph~repo] [![petgraph~lib.rs][c~petgraph~lib.rs~badge]][c~petgraph~lib.rs]{{hi:petgraph}}{{hi:BFS}}{{hi:Graph traversal}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}

Breadth-First Search (BFS){{hi:Breadth-First Search}} visits all neighbors of a node before visiting their neighbors, level by level. It is commonly used to find the shortest path in an unweighted graph or to explore all reachable nodes from a starting point.

```rust,editable,noplayground
{{#include ../../../crates/cats/algorithms/examples/graph/petgraph_bfs.rs:example}}
```

## Traverse a Graph with Depth-First Search (DFS) {#petgraph-dfs}

[![petgraph][c~petgraph~docs~badge]][c~petgraph~docs] [![petgraph~crates.io][c~petgraph~crates.io~badge]][c~petgraph~crates.io] [![petgraph~repo][c~petgraph~repo~badge]][c~petgraph~repo] [![petgraph~lib.rs][c~petgraph~lib.rs~badge]][c~petgraph~lib.rs]{{hi:petgraph}}{{hi:DFS}}{{hi:Graph traversal}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}

Depth-First Search (DFS){{hi:Depth-First Search}} explores as far as possible along each branch before backtracking. It is useful for cycle detection, generating topological orderings, and solving maze or pathfinding problems.

```rust,editable,noplayground
{{#include ../../../crates/cats/algorithms/examples/graph/petgraph_dfs.rs:example}}
```

## Find Shortest Paths with Dijkstra's Algorithm {#petgraph-dijkstra}

[![petgraph][c~petgraph~docs~badge]][c~petgraph~docs] [![petgraph~crates.io][c~petgraph~crates.io~badge]][c~petgraph~crates.io] [![petgraph~repo][c~petgraph~repo~badge]][c~petgraph~repo] [![petgraph~lib.rs][c~petgraph~lib.rs~badge]][c~petgraph~lib.rs]{{hi:petgraph}}{{hi:Dijkstra}}{{hi:Shortest path}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}

Dijkstra's algorithm{{hi:Dijkstra's algorithm}} finds the shortest path from a source node to all other reachable nodes in a weighted graph with non-negative edge weights. It is widely used in routing protocols, GPS navigation, and network optimization.

```rust,editable,noplayground
{{#include ../../../crates/cats/algorithms/examples/graph/petgraph_dijkstra.rs:example}}
```

## Sort Tasks Topologically in a DAG {#petgraph-toposort}

[![petgraph][c~petgraph~docs~badge]][c~petgraph~docs] [![petgraph~crates.io][c~petgraph~crates.io~badge]][c~petgraph~crates.io] [![petgraph~repo][c~petgraph~repo~badge]][c~petgraph~repo] [![petgraph~lib.rs][c~petgraph~lib.rs~badge]][c~petgraph~lib.rs]{{hi:petgraph}}{{hi:Topological sort}}{{hi:DAG}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}

Topological sort{{hi:Topological sort}} produces a linear ordering of the nodes of a Directed Acyclic Graph (DAG) such that for every directed edge from node `u` to node `v`, `u` appears before `v` in the ordering. This is essential for task scheduling, dependency resolution (e.g., in build systems or package managers), and determining the order of operations.

```rust,editable,noplayground
{{#include ../../../crates/cats/algorithms/examples/graph/petgraph_toposort.rs:example}}
```

## See Also {#see-also .skip}

- [`pathfinding`][c~pathfinding~docs]↗{{hi:pathfinding}} provides pathfinding algorithms including A*, Dijkstra, BFS, DFS, and more.
- [`petgraph`][c~petgraph~docs]↗ also supports minimum spanning tree (Kruskal's algorithm), Bellman-Ford, Floyd-Warshall, and more.

## Related Topics {#related-topics .skip}

- [[data-structures | Data Structures]].
- [[graphs | Graphs]] (petgraph data structures).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
