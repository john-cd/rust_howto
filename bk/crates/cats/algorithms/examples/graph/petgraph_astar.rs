#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Shortest path search on a weighted graph using the A* algorithm with
//! `petgraph`.
//!
//! ### What is the A* Search Algorithm?
//! A* (pronounced "A-star") is an informed graph traversal and pathfinding
//! algorithm that computes the shortest path between a starting node and a
//! target goal node.
//!
//! ### How Does A* Work?
//! A* extends Dijkstra's algorithm by using a heuristic function $h(n)$ to
//! guide its search toward the target. It evaluates candidate nodes using the
//! cost function:
//! $$f(n) = g(n) + h(n)$$
//! - **$g(n)$**: The exact cost incurred to travel from the start node to node
//!   $n$.
//! - **$h(n)$**: An estimated cost (heuristic) to travel from node $n$ to the
//!   goal.
//! - **$f(n)$**: The estimated total cost of a path passing through node $n$.
//!
//! ### Heuristic Admissibility
//! For A* to guarantee finding the optimal (shortest) path, the heuristic
//! function $h(n)$ must be **admissible**. An admissible heuristic never
//! overestimates the true remaining cost to reach the goal. Common admissible
//! heuristics include Manhattan distance for grid movements and Euclidean
//! straight-line distance.
//!
//! ### Comparison with Dijkstra's Algorithm
//! Dijkstra's algorithm searches blindly in all directions ($h(n) = 0$). By
//! contrast, A* uses $h(n)$ to prioritize exploring nodes closest to the
//! goal, dramatically reducing search effort on large graphs.
//!
//! ### Typical Use Cases
//! - Game development for non-player character (NPC) pathfinding.
//! - GPS and map routing for vehicles and pedestrians.
//! - Robot motion planning and spatial navigation.

use petgraph::Graph;
use petgraph::algo::astar;

/// Demonstrates finding the shortest path on a weighted graph using A* and
/// Manhattan distance.
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

    let goal = e;

    // Run A* search from starting node `a` to goal node `e`.
    // The closures define:
    // 1. Goal test (`|finish| finish == goal`).
    // 2. Edge cost extractor (`|edge| *edge.weight()`).
    // 3. Admissible heuristic (`|node| ...` calculating Manhattan distance).
    let path = astar(
        &graph,
        a,
        |finish| finish == goal,
        |edge| *edge.weight(),
        |node| {
            let (x, y) = graph[node];
            let (goal_x, goal_y) = graph[goal];
            x.abs_diff(goal_x) + y.abs_diff(goal_y)
        },
    );

    println!("Shortest path from A to E:");
    if let Some((cost, path_nodes)) = path {
        println!("  Cost: {cost}");
        for node in path_nodes {
            println!("  Node: {:?}", graph[node]);
        }
        // Verify path cost: A -> D (2) -> E (1) = total cost 3.
        assert_eq!(cost, 3);
    } else {
        println!("No path found.");
    }
}

// ANCHOR_END: example

/// Runs the `petgraph` A* example by calling `main()`.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_petgraph_astar_example_entry_point() {
        main();
    }
}
