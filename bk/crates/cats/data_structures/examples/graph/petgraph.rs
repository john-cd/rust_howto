#![allow(dead_code)]
// ANCHOR: example
use petgraph::Directed;
use petgraph::Undirected;
use petgraph::algo::dijkstra;
use petgraph::dot::Config;
use petgraph::dot::Dot;
use petgraph::graph::Graph;

fn main() {
    // Create a new directed graph:
    let mut graph = Graph::<&str, &str, Directed>::new();

    // Add nodes to the graph:
    let a = graph.add_node("Node A");
    let b = graph.add_node("Node B");
    let c = graph.add_node("Node C");
    let d = graph.add_node("Node D");

    // Add edges between nodes:
    graph.add_edge(a, b, "Edge AB");
    graph.add_edge(b, c, "Edge BC");
    graph.add_edge(c, d, "Edge CD");
    graph.add_edge(d, a, "Edge DA");

    // Find the shortest path from `b` using `1` as the cost for all edges:
    let node_map = dijkstra(&graph, b, Some(d), |_| 1);
    println!("{node_map:?}");

    // Print the graph in DOT format:
    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    // Traverse the graph:
    println!("\nTraversing the graph:");
    for node in graph.node_indices() {
        println!("Node: {:?}", graph[node]);
        for neighbor in graph.neighbors(node) {
            println!("  Neighbor: {:?}", graph[neighbor]);
        }
    }

    // Create an undirected graph:
    let mut undirected_graph =
        Graph::<&str, &str, Undirected>::new_undirected();

    // Add nodes to the undirected graph:
    let e = undirected_graph.add_node("Node E");
    let f = undirected_graph.add_node("Node F");
    let g = undirected_graph.add_node("Node G");

    // Add edges to the undirected graph:
    undirected_graph.add_edge(e, f, "Edge EF");
    undirected_graph.add_edge(e, g, "Edge EG");

    let _undirected_graph = undirected_graph;
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
