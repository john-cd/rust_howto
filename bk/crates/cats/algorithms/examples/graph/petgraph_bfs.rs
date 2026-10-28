// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Breadth-First Search (BFS) traversal of a graph using `petgraph`.
//!
//! ### What is Breadth-First Search (BFS)?
//! Breadth-First Search is an unweighted graph traversal algorithm that
//! explores vertices layer by layer (level order) outward from a starting
//! source node.
//!
//! ### How Does It Work?
//! BFS uses a First-In, First-Out (FIFO) queue data structure to track frontier
//! nodes. It visits all immediate 1-hop neighbors of the current node before
//! moving on to 2-hop neighbors, 3-hop neighbors, and so on.
//!
//! ### Key Properties & Shortest Path Guarantee
//! - **Unweighted Shortest Path**: On unweighted graphs (where all edge costs
//!   are equal), BFS is guaranteed to discover the path with the minimum number
//!   of edges from the source to any reachable destination.
//! - **Level-Order Exploration**: Ensures systematically visiting shallower
//!   nodes before deeper nodes.
//!
//! ### Typical Use Cases
//! - Finding shortest paths in unweighted networks or mazes.
//! - Web crawlers discovering pages level-by-level from a seed URL.
//! - Identifying degrees of separation in social media graphs.

use petgraph::Graph;
use petgraph::visit::Bfs;

/// Demonstrates level-order graph traversal using Breadth-First Search (BFS).
fn main() {
    // Create a directed unweighted graph.
    let mut graph = Graph::<&str, ()>::new();

    // Add nodes (vertices) to the graph.
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");
    let e = graph.add_node("E");

    // Add directed edges.
    graph.add_edge(a, b, ());
    graph.add_edge(a, c, ());
    graph.add_edge(b, d, ());
    graph.add_edge(c, d, ());
    graph.add_edge(d, e, ());

    // Perform BFS traversal starting from source node `a`.
    let mut bfs = Bfs::new(&graph, a);
    let mut visited: Vec<&str> = Vec::new();
    while let Some(node) = bfs.next(&graph) {
        visited.push(graph[node]);
    }

    println!("BFS traversal order: {visited:?}");

    // BFS explores level-by-level: Node A (level 0), then B and C (level 1),
    // followed by D (level 2), and finally E (level 3).
    assert_eq!(visited[0], "A");
    assert!(visited.contains(&"B"));
    assert!(visited.contains(&"C"));

    // Verify ordering invariants across exploration levels.
    let b_position = visited
        .iter()
        .position(|&s| s == "B")
        .expect("expected item should appear in the traversal.");
    let c_position = visited
        .iter()
        .position(|&s| s == "C")
        .expect("expected item should appear in the traversal.");
    let d_position = visited
        .iter()
        .position(|&s| s == "D")
        .expect("expected item should appear in the traversal.");
    let e_position = visited
        .iter()
        .position(|&s| s == "E")
        .expect("expected item should appear in the traversal.");

    assert!(d_position > b_position && d_position > c_position);
    assert!(e_position > d_position);
}
// ANCHOR_END: example

/// Runs the `petgraph` BFS example by calling `main()`.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_petgraph_bfs_example_entry_point() {
        main();
    }
}
