// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Finds a lowest-cost route through an explicitly stored graph.

use petgraph::Graph;
use petgraph::algo::astar;
use petgraph::graph::NodeIndex;

/// Demonstrates finding the shortest path on a weighted graph using A* and
/// Manhattan distance.
///
/// # Panics
///
/// Panics if the connected fixture fails to produce either expected shortest route.
fn main() {
    // Create an explicit directed graph where each node stores 2D grid
    // coordinates `(x, y)` and each edge stores a traversal cost (`u32`).
    let mut graph = Graph::<(i32, i32), u32>::new();

    // Add nodes representing locations on a 2D plane.
    let a = graph.add_node((0, 0));
    let b = graph.add_node((1, 0));
    let c = graph.add_node((2, 0));
    let d = graph.add_node((1, 1));
    let e = graph.add_node((2, 1));

    // Connect nodes with directed edges and assigned costs (weights).
    graph.add_edge(a, b, 1);
    graph.add_edge(b, c, 1);
    graph.add_edge(a, d, 2);
    graph.add_edge(d, e, 1);
    graph.add_edge(c, e, 1);

    let (cost, route) =
        shortest_route(&graph, a, e).expect("the connected fixture has two shortest routes to E");
    assert_eq!(cost, 3);
    assert!(route == [a, d, e] || route == [a, b, c, e]);

    println!("Shortest path from A to E (cost {cost}):");
    for node in route {
        println!("  Node: {:?}", graph[node]);
    }
}

/// Finds the cost and node sequence, or returns `None` if the goal is unreachable.
///
/// Each edge must cost at least its Manhattan displacement for this heuristic
/// to be admissible.
fn shortest_route(
    graph: &Graph<(i32, i32), u32>,
    start: NodeIndex,
    goal: NodeIndex,
) -> Option<(u32, Vec<NodeIndex>)> {
    astar(
        graph,
        start,
        |node| node == goal,
        |edge| *edge.weight(),
        |node| {
            let (x, y) = graph[node];
            let (goal_x, goal_y) = graph[goal];
            x.abs_diff(goal_x) + y.abs_diff(goal_y)
        },
    )
}
// ANCHOR_END: example

/// Demonstrates A* shortest-path search on a weighted graph with Manhattan distance.
pub fn run() {
    main();
}

/// Tests behavior implemented by the surrounding module.
#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the connected fixture produces an optimal route.
    #[test]
    fn test_petgraph_astar_example_entry_point() {
        main();
    }

    /// Returns no path for a goal with no incoming edges.
    #[test]
    fn unreachable_goal_has_no_route() {
        let mut graph = Graph::new();
        let start = graph.add_node((0, 0));
        let goal = graph.add_node((1, 0));
        assert_eq!(shortest_route(&graph, start, goal), None);
    }
}
