// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Minimum Spanning Tree (MST) computation on an undirected graph using
//! `petgraph`.
//!
//! ### What is a Minimum Spanning Tree (MST)?
//! A Minimum Spanning Tree is a subset of edges from a connected, weighted
//! undirected graph that connects all vertices together with the minimum
//! possible total edge weight, without introducing any cycles.
//!
//! ### Underlying Algorithm (Kruskal's Algorithm)
//! `petgraph` uses **Kruskal's algorithm** internally:
//! 1. Sorts all graph edges in non-decreasing order of their weights.
//! 2. Iteratively inspects edges from lightest to heaviest.
//! 3. Adds an edge to the spanning tree if and only if it connects two
//!    previously disconnected components (using a Disjoint-Set / Union-Find
//!    structure), preventing cycle formation.
//!
//! ### Purpose & Use Cases
//! - **Network Infrastructure Design**: Laying telecommunication cables,
//!   electrical grids, or water pipelines to connect all locations at minimal
//!   construction cost.
//! - **Approximation Algorithms**: Serving as a building block for TSP
//!   (Travelling Salesperson Problem) heuristics and clustering algorithms.

use petgraph::Graph;
use petgraph::algo::min_spanning_tree;
use petgraph::data::FromElements as _;
use petgraph::dot::Config;
use petgraph::dot::Dot;

/// Demonstrates calculating the Minimum Spanning Tree (MST) of an undirected
/// weighted graph using Kruskal's algorithm.
fn main() {
    // Create an undirected graph with string node labels and numeric edge
    // weights (`u32`).
    let mut graph = Graph::<&str, u32, petgraph::Undirected>::new_undirected();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");
    let e = graph.add_node("E");

    // Add weighted undirected edges.
    graph.add_edge(a, b, 1);
    graph.add_edge(a, c, 3);
    graph.add_edge(b, c, 1);
    graph.add_edge(b, d, 4);
    graph.add_edge(c, d, 1);
    graph.add_edge(d, e, 1);
    graph.add_edge(c, e, 5);

    // Compute the Minimum Spanning Tree using Kruskal's algorithm.
    // `min_spanning_tree` returns an iterator over tree elements (nodes and
    // selected edges).
    let mst_elements = min_spanning_tree(&graph);
    let mst_graph =
        Graph::<&str, u32, petgraph::Undirected>::from_elements(mst_elements);

    // Print the resulting MST graph rendered in Graphviz DOT format.
    println!(
        "MST in DOT format:\n{:?}",
        Dot::with_config(&mst_graph, &[Config::EdgeNoLabel])
    );

    // Verify minimum total edge weight:
    // Selected edges: A-B (1) + B-C (1) + C-D (1) + D-E (1) = total weight 4.
    let total_weight: u32 = mst_graph.edge_weights().sum();
    println!("Total MST weight: {total_weight}");
    assert_eq!(total_weight, 4);
}

// ANCHOR_END: example

/// Runs the `petgraph` Minimum Spanning Tree example by calling `main()`.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_petgraph_minimum_spanning_tree_example() {
        main();
    }
}
