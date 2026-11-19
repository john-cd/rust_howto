<!-- TODO Reviewed ??-??-2026. -->

# Graph Algorithms

{{#include graph_algorithms.incl.md}}

Graph [algorithms][p~category-algorithms]{{hi:algorithms}} operate on [data structures][p~category-data-structures]{{hi:data-structures}} consisting of nodes (vertices) connected by edges. They are used in a wide variety of applications, including network routing, social network analysis, dependency resolution, and pathfinding in [games][cat~games]↗{{hi:games}}.

The [`petgraph`][c~petgraph~docs]↗{{hi:petgraph}} crate provides graph data structures and a rich set of graph algorithms. It supports directed and undirected graphs, and provides implementations of many classic algorithms.

## Traverse a Graph with Breadth-First Search (BFS) {#petgraph-bfs}

{{#badges petgraph algorithms=Algorithms}}{{hi:BFS}}{{hi:Graph traversal}}

Breadth-First Search (BFS){{hi:Breadth-First Search}} visits all neighbors of a node before visiting their neighbors, level by level. It is commonly used to find the shortest path in an unweighted graph or to explore all reachable nodes from a starting point.

```rust,editable,noplayground
{{#include ../../../crates/cats/algorithms/examples/graph/petgraph_bfs.rs:example}}
```

## Traverse a Graph with Depth-First Search (DFS) {#petgraph-dfs}

{{#badges petgraph algorithms=Algorithms}}{{hi:DFS}}{{hi:Graph traversal}}

Depth-First Search (DFS){{hi:Depth-First Search}} explores as far as possible along each branch before backtracking. It is useful for cycle detection, generating topological orderings, and solving maze or pathfinding problems.

```rust,editable,noplayground
{{#include ../../../crates/cats/algorithms/examples/graph/petgraph_dfs.rs:example}}
```

## Find Shortest Paths with Dijkstra's Algorithm {#petgraph-dijkstra}

{{#badges petgraph algorithms=Algorithms}}{{hi:Dijkstra}}{{hi:Shortest path}}

Dijkstra's algorithm{{hi:Dijkstra's algorithm}} finds the shortest path from a source node to all other reachable nodes in a weighted graph with non-negative edge weights. It is widely used in routing protocols, GPS navigation, and network optimization.

```rust,editable,noplayground
{{#include ../../../crates/cats/algorithms/examples/graph/petgraph_dijkstra.rs:example}}
```

## Sort Tasks Topologically in a DAG {#petgraph-toposort}

{{#badges petgraph algorithms=Algorithms}}{{hi:Topological sort}}{{hi:DAG}}

Topological sort{{hi:Topological sort}} produces a linear ordering of the nodes of a Directed Acyclic Graph (DAG) such that for every directed edge from node `u` to node `v`, `u` appears before `v` in the ordering. This is essential for task scheduling, dependency resolution (e.g., in build systems or package managers), and determining the order of operations.

```rust,editable,noplayground
{{#include ../../../crates/cats/algorithms/examples/graph/petgraph_toposort.rs:example}}
```

## Find Shortest Paths with A* Search {#petgraph-astar}

{{#badges petgraph algorithms=Algorithms}}{{hi:A* Search}}{{hi:Shortest path}}

A\* Search{{hi:A* Search}} chooses a candidate using the cost already incurred, `g`, plus an estimate of the remaining cost, `h`. In this graph, going from A to B gives `g = 1` and `h = 2`. Going from A to D gives `g = 2` and `h = 1`. Both candidates have score 3, and either can lead to an optimal route.

To guarantee an optimal result with [`petgraph::algo::astar`](https://docs.rs/petgraph/latest/petgraph/algo/astar/fn.astar.html), use nonnegative edge costs and an estimate that never exceeds the cheapest remaining path. Here every edge costs at least the Manhattan distance it covers, so that distance is a lower bound on the remaining cost. A diagonal from `(0, 0)` to `(1, 1)` costing 1 would invalidate that bound. When you cannot justify a lower bound, use zero. This makes A* prioritize accumulated costs like [Dijkstra's algorithm](#petgraph-dijkstra).

This example stores the graph explicitly. The [grid recipe](#pathfinding-astar) generates its neighbors on demand.

```rust,editable,noplayground
{{#include ../../../crates/cats/algorithms/examples/graph/petgraph_astar.rs:example}}
```

## Find Shortest Paths on a Grid with `pathfinding` {#pathfinding-astar}

{{#badges pathfinding algorithms=Algorithms}}{{hi:A* Search}}{{hi:Shortest path}}{{hi:Grid}}

The [`pathfinding`][c~pathfinding~docs]↗{{hi:pathfinding}} crate provides many graph search algorithms. It is particularly well-suited for pathfinding on grids or in situations where the graph is defined implicitly by a successor function rather than being stored in memory.

```rust,editable,noplayground
{{#include ../../../crates/cats/algorithms/examples/graph/pathfinding_astar.rs:example}}
```

## Minimum Spanning Tree with `petgraph` {#petgraph-mst}

{{#badges petgraph algorithms=Algorithms}}{{hi:MST}}{{hi:Minimum Spanning Tree}}{{hi:Kruskal}}

A Minimum Spanning Tree (MST){{hi:Minimum Spanning Tree}} is a subset of the edges of a connected, edge-weighted undirected graph that connects all the vertices together, without any cycles and with the minimum possible total edge weight.

```rust,editable,noplayground
{{#include ../../../crates/cats/algorithms/examples/graph/petgraph_mst.rs:example}}
```

## See Also {#see-also .skip}

- [`petgraph`][c~petgraph~docs]↗ also provides Bellman-Ford, Floyd-Warshall, and other graph algorithms.

## Related Topics {#related-topics .skip}

- [[data-structures | Data Structures]].
- [[graphs | Graphs]] (petgraph data structures).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
