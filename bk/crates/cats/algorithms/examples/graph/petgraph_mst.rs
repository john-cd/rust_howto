#![allow(dead_code)]
// ANCHOR: example
use petgraph::Graph;
use petgraph::algo::min_spanning_tree;
use petgraph::data::FromElements;
use petgraph::dot::{Config, Dot};

/// Demonstrates finding the Minimum Spanning Tree (MST) of a graph using
/// Kruskal's algorithm in `petgraph`.
fn main() {
    // Create an undirected graph with weighted edges.
    let mut graph = Graph::<&str, u32, petgraph::Undirected>::new_undirected();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");
    let e = graph.add_node("E");

    graph.add_edge(a, b, 1);
    graph.add_edge(a, c, 3);
    graph.add_edge(b, c, 1);
    graph.add_edge(b, d, 4);
    graph.add_edge(c, d, 1);
    graph.add_edge(d, e, 1);
    graph.add_edge(c, e, 5);

    // Compute the Minimum Spanning Tree.
    // `min_spanning_tree` returns an iterator of `Element`s (nodes and edges).
    let mst_elements = min_spanning_tree(&graph);
    let mst_graph = Graph::<&str, u32, petgraph::Undirected>::from_elements(mst_elements);

    println!("MST in DOT format:\n{:?}", Dot::with_config(&mst_graph, &[Config::EdgeNoLabel]));

    // Total weight of MST should be 1 (A-B) + 1 (B-C) + 1 (C-D) + 1 (D-E) = 4.
    let total_weight: u32 = mst_graph.edge_weights().sum();
    println!("Total MST weight: {}", total_weight);
    assert_eq!(total_weight, 4);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
