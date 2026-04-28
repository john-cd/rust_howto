#![allow(dead_code)]
// ANCHOR: example
use petgraph::Graph;
use petgraph::visit::Bfs;

/// Demonstrates Breadth-First Search (BFS) traversal using `petgraph`.
///
/// BFS visits all neighbors of a node before visiting neighbors of neighbors.
/// It is useful for finding the shortest path in an unweighted graph.
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
    graph.add_edge(c, d, ());
    graph.add_edge(d, e, ());

    // Perform BFS starting from node `a`.
    let mut bfs = Bfs::new(&graph, a);
    let mut visited: Vec<&str> = Vec::new();
    while let Some(node) = bfs.next(&graph) {
        visited.push(graph[node]);
    }

    println!("BFS traversal order: {:?}", visited);
    // BFS visits level by level: A first, then B and C (in any order), then D,
    // then E.
    assert_eq!(visited[0], "A");
    assert!(visited.contains(&"B"));
    assert!(visited.contains(&"C"));
    // D must appear after both B and C; E must appear last.
    let b_position = visited.iter().position(|&s| s == "B").unwrap();
    let c_position = visited.iter().position(|&s| s == "C").unwrap();
    let d_position = visited.iter().position(|&s| s == "D").unwrap();
    let e_position = visited.iter().position(|&s| s == "E").unwrap();
    assert!(d_position > b_position && d_position > c_position);
    assert!(e_position > d_position);
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        main();
    }
}
