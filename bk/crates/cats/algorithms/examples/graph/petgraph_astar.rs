#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! A* shortest path in a `petgraph` graph.

use petgraph::Graph;
use petgraph::algo::astar;

/// Demonstrates A* shortest path algorithm using `petgraph`.
fn main() {
    let mut graph = Graph::<(i32, i32), u32>::new();

    let a = graph.add_node((0, 0));
    let b = graph.add_node((1, 0));
    let c = graph.add_node((2, 0));
    let d = graph.add_node((1, 1));
    let e = graph.add_node((2, 1));

    graph.add_edge(a, b, 1);
    graph.add_edge(b, c, 1);
    graph.add_edge(a, d, 2);
    graph.add_edge(d, e, 1);
    graph.add_edge(c, e, 1);

    let goal = e;
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
        assert_eq!(cost, 3);
    } else {
        println!("No path found.");
    }
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the petgraph astar example through its entry point.
    #[test]
    fn test_petgraph_astar_example_entry_point() {
        main();
    }
}
