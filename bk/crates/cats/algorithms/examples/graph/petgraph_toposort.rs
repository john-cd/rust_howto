#![allow(dead_code)]
// ANCHOR: example
use petgraph::Graph;
use petgraph::algo::toposort;

/// Demonstrates topological sorting of a DAG (Directed Acyclic Graph) using
/// `petgraph`.
///
/// Topological sort produces a linear ordering of vertices such that for every
/// directed edge (u, v), vertex u comes before v in the ordering.
/// It is used in task scheduling, dependency resolution, and build systems.
fn main() {
    // Create a DAG representing task dependencies.
    // An edge A -> B means "A must be completed before B".
    let mut graph = Graph::<&str, ()>::new();

    // Add tasks as nodes.
    let fetch_data = graph.add_node("Fetch Data");
    let validate = graph.add_node("Validate");
    let transform = graph.add_node("Transform");
    let load = graph.add_node("Load");
    let report = graph.add_node("Report");

    // Add dependency edges.
    graph.add_edge(fetch_data, validate, ());
    graph.add_edge(validate, transform, ());
    graph.add_edge(transform, load, ());
    graph.add_edge(load, report, ());
    graph.add_edge(validate, report, ()); // Validate also feeds Report directly.

    // Compute a topological ordering.
    // Returns `Err` if the graph contains a cycle.
    match toposort(&graph, None) {
        Ok(order) => {
            println!("Topological order (tasks to execute in sequence):");
            for node in &order {
                println!("  - {}", graph[*node]);
            }
            // Verify that "Fetch Data" comes before "Validate"
            // and "Validate" comes before "Load".
            let pos: Vec<&str> = order.iter().map(|n| graph[*n]).collect();
            let fetch_position =
                pos.iter().position(|&s| s == "Fetch Data").unwrap();
            let validate_position =
                pos.iter().position(|&s| s == "Validate").unwrap();
            let load_position = pos.iter().position(|&s| s == "Load").unwrap();
            assert!(fetch_position < validate_position);
            assert!(validate_position < load_position);
        }
        Err(_) => println!("Graph contains a cycle!"),
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
