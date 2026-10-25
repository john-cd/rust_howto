// ANCHOR: example
use petgraph::algo::is_cyclic_directed;
use petgraph::algo::tarjan_scc;
use petgraph::graph::DiGraph;

/// Creates a dependency graph with two mutually dependent tasks.
fn dependency_graph() -> DiGraph<&'static str, ()> {
    let mut graph = DiGraph::new();
    let parse = graph.add_node("parse");
    let validate = graph.add_node("validate");
    let publish = graph.add_node("publish");
    graph.add_edge(parse, validate, ());
    graph.add_edge(validate, parse, ());
    graph.add_edge(validate, publish, ());
    graph
}

/// Returns the graph's cyclic strongly connected components as task names.
fn cyclic_components(
    graph: &DiGraph<&'static str, ()>,
) -> Vec<Vec<&'static str>> {
    let mut cycles = tarjan_scc(graph)
        .into_iter()
        .filter(|component| {
            component.len() > 1
                || component
                    .first()
                    .is_some_and(|node| graph.contains_edge(*node, *node))
        })
        .map(|component| {
            let mut names = component
                .into_iter()
                .map(|node| graph[node])
                .collect::<Vec<_>>();
            names.sort_unstable();
            names
        })
        .collect::<Vec<_>>();
    cycles.sort();
    cycles
}

/// Prints whether the graph has a cycle and the names in each cyclic component.
fn main() {
    let graph = dependency_graph();
    println!("Has cycle: {}", is_cyclic_directed(&graph));
    for component in cyclic_components(&graph) {
        println!("Cycle members: {component:?}");
    }
}
// ANCHOR_END: example

/// Runs the example from the chapter's example runner.
pub fn run() {
    main();
}

/// Verifies cycle detection identifies all tasks in the cycle.
#[test]
fn test() {
    let graph = dependency_graph();
    assert!(is_cyclic_directed(&graph));
    assert_eq!(cyclic_components(&graph), vec![vec!["parse", "validate"]]);
}
