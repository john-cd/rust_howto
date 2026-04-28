#![allow(dead_code)]
// ANCHOR: example
use petgraph::Graph;
use petgraph::visit::Dfs;

/// Demonstrates Depth-First Search (DFS) traversal using `petgraph`.
///
/// DFS explores as far as possible along each branch before backtracking.
/// It is useful for cycle detection, topological sorting, and pathfinding.
fn main() {
    // Create a directed graph.
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
    graph.add_edge(c, e, ());

    // Perform DFS starting from node `a`.
    let mut dfs = Dfs::new(&graph, a);
    let mut visited: Vec<&str> = Vec::new();
    while let Some(node) = dfs.next(&graph) {
        visited.push(graph[node]);
    }

    println!("DFS traversal order: {:?}", visited);
    // DFS goes deep first: A -> B -> D -> C -> E (or A -> C -> E -> B -> D,
    // depending on adjacency list order)
    assert!(visited.contains(&"A"));
    assert!(visited.contains(&"B"));
    assert!(visited.contains(&"C"));
    assert!(visited.contains(&"D"));
    assert!(visited.contains(&"E"));
    assert_eq!(visited.len(), 5);
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        main();
    }
}
