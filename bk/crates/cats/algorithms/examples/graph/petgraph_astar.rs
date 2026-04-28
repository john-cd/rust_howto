#![allow(dead_code)]
// ANCHOR: example
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

    let path = astar(
        &graph,
        a,
        |finish| finish == e,
        |e| *e.weight(),
        |node| {
            let (x, y) = graph[node];
            let (ex, ey) = graph[e];
            (x.abs_diff(ex) + y.abs_diff(ey)) as u32
        },
    );

    println!("Shortest path from A to E:");
    if let Some((cost, path_nodes)) = path {
        println!("  Cost: {}", cost);
        for node in path_nodes {
            println!("  Node: {:?}", graph[node]);
        }
        assert_eq!(cost, 3);
    } else {
        println!("No path found.");
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
