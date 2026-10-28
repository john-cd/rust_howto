#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Single-source shortest path calculation on a weighted graph using Dijkstra's
//! algorithm in `petgraph`.
//!
//! ### What is Dijkstra's Algorithm?
//! Dijkstra's algorithm is a fundamental graph search algorithm that calculates
//! the shortest path from a single source node to all other reachable nodes in
//! a weighted graph.
//!
//! ### How Does It Work?
//! The algorithm maintains a priority queue (min-heap) of tentative distance
//! costs from the source node. At each step, it greedily selects the node with
//! the smallest tentative distance, explores its outgoing edges, and updates
//! (relaxes) neighbor distances if a shorter path is found.
//!
//! ### Key Invariants & Constraints
//! - **Non-Negative Edge Weights**: Dijkstra's algorithm requires all edge
//!   weights to be non-negative ($\ge 0$). Negative edge weights break greedy
//!   relaxation invariants (use Bellman-Ford for negative weights).
//! - **Dijkstra vs. A***: Dijkstra explores outwards uniformly in all
//!   directions. A* is an extension that adds a heuristic estimate $h(n)$ to
//!   direct search towards a specific target node faster.
//!
//! ### Purpose & Use Cases
//! - Transportation network routing (e.g., finding shortest driving distances).
//! - Network packet routing protocols (e.g., OSPF, IS-IS).
//! - Degree of separation analysis in social networks.

use petgraph::Graph;
use petgraph::algo::dijkstra;

/// Demonstrates Dijkstra's single-source shortest path search on a weighted
/// directed graph.
fn main() {
    // Create a weighted directed graph where node weights are string labels
    // and edge weights are numeric costs (`u32`).
    let mut graph = Graph::<&str, u32>::new();

    // Add nodes representing locations or vertices.
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");
    let e = graph.add_node("E");

    // Add directed weighted edges (source, target, weight).
    graph.add_edge(a, b, 4);
    graph.add_edge(a, c, 2);
    graph.add_edge(b, d, 5);
    graph.add_edge(c, b, 1);
    graph.add_edge(c, d, 8);
    graph.add_edge(c, e, 10);
    graph.add_edge(d, e, 2);

    // Compute shortest path costs from source node `a` to all reachable nodes.
    // The closure `|e| *e.weight()` provides edge traversal cost to the
    // algorithm.
    let distances = dijkstra(&graph, a, None, |e| *e.weight());

    println!("Shortest distances from A:");
    for (node, distance) in &distances {
        println!("  A -> {}: {}", graph[*node], distance);
    }

    // Verify calculated path distances:
    // A -> A: cost 0.
    // A -> C: cost 2.
    // A -> C -> B: cost 2 + 1 = 3 (cheaper than direct A -> B cost 4).
    // A -> C -> B -> D: cost 3 + 5 = 8.
    // A -> C -> B -> D -> E: cost 8 + 2 = 10.
    assert_eq!(distances[&a], 0);
    assert_eq!(distances[&b], 3);
    assert_eq!(distances[&c], 2);
    assert_eq!(distances[&d], 8);
    assert_eq!(distances[&e], 10);
}

// ANCHOR_END: example

/// Runs the `petgraph` Dijkstra example by calling `main()`.
pub fn run() {
    main();
}

#[cfg(test)]
/// Unit tests for the `petgraph` Dijkstra algorithm example.
mod tests {
    use super::*;

    /// Runs the `petgraph` Dijkstra example through its `main()` entry point.
    #[test]
    fn test_petgraph_dijkstra_example_entry_point() {
        main();
    }
}
