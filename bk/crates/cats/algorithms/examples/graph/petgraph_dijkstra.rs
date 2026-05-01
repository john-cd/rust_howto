#![allow(dead_code)]
// ANCHOR: example
use petgraph::Graph;
use petgraph::algo::dijkstra;

/// Demonstrates Dijkstra's shortest path algorithm using `petgraph`.
///
/// Dijkstra's algorithm finds the shortest path from a source node to all
/// other nodes in a weighted graph with non-negative edge weights.
fn main() {
    // Create a weighted directed graph.
    // Edge weights represent distances or costs.
    let mut graph = Graph::<&str, u32>::new();

    // Add nodes (vertices).
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

    // Compute shortest distances from node `a` to all reachable nodes.
    // The closure |e| *e.weight() provides the edge cost.
    let distances = dijkstra(&graph, a, None, |e| *e.weight());

    println!("Shortest distances from A:");
    for (node, distance) in &distances {
        println!("  A -> {}: {}", graph[*node], distance);
    }

    // Shortest path: A -> C (cost 2) -> B (cost 2+1=3) -> D (cost 3+5=8)
    // -> E (cost 8+2=10)
    assert_eq!(distances[&a], 0);
    assert_eq!(distances[&b], 3); // A -> C -> B: 2 + 1 = 3
    assert_eq!(distances[&c], 2); // A -> C: 2
    assert_eq!(distances[&d], 8); // A -> C -> B -> D: 2 + 1 + 5 = 8
    assert_eq!(distances[&e], 10); // A -> C -> B -> D -> E: 8 + 2 = 10
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
