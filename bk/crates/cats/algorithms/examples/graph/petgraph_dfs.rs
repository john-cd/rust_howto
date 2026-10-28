// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Depth-First Search (DFS) traversal of a graph using `petgraph`.
//!
//! ### What is Depth-First Search (DFS)?
//! Depth-First Search is a graph traversal algorithm that explores as far down
//! a branch as possible along each path before backtracking.
//!
//! ### How Does It Work?
//! DFS manages exploration using a Last-In, First-Out (LIFO) call stack or
//! explicit stack buffer. It follows unvisited outgoing edges sequentially
//! until it encounters a dead end or previously visited node, then backtracks
//! to explore remaining unvisited branch choices.
//!
//! ### Key Properties & Comparison with BFS
//! - **Memory Efficiency**: In deep sparse graphs, DFS uses memory proportional
//!   to maximum search depth $O(V)$, whereas BFS memory grows with frontier
//!   width $O(V)$.
//! - **Non-Optimal Paths**: Unlike BFS, DFS does **not** guarantee finding the
//!   shortest path on unweighted graphs.
//!
//! ### Typical Use Cases
//! - Detecting cycles in directed or undirected graphs.
//! - Computing topological ordering of directed acyclic graphs.
//! - Topological maze generation and dead-end path discovery.
//! - Identifying strongly connected components.

use petgraph::Graph;
use petgraph::visit::Dfs;

/// Demonstrates depth-first graph traversal and branch exploration order using
/// `petgraph`.
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
    graph.add_edge(a, c, ());
    graph.add_edge(a, b, ());
    graph.add_edge(b, d, ());
    graph.add_edge(c, e, ());

    // Perform DFS traversal starting from node `a`.
    let mut dfs = Dfs::new(&graph, a);
    let mut visited: Vec<&str> = Vec::new();
    while let Some(node) = dfs.next(&graph) {
        visited.push(graph[node]);
    }

    println!("DFS traversal order: {visited:?}");

    // Verify traversal coverage and branch traversal order.
    assert!(visited.contains(&"A"));
    assert!(visited.contains(&"B"));
    assert!(visited.contains(&"C"));
    assert!(visited.contains(&"D"));
    assert!(visited.contains(&"E"));
    assert_eq!(visited.len(), 5);

    // Node B's outgoing neighbor D is visited immediately after B before
    // backtracking to explore C.
    let b_position = visited.iter().position(|&node| node == "B");
    let d_position = visited.iter().position(|&node| node == "D");
    assert_eq!(d_position, b_position.map(|position| position + 1));
}

// ANCHOR_END: example

/// Runs the `petgraph` DFS example by calling `main()`.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_petgraph_dfs_example_entry_point() {
        main();
    }
}
