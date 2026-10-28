// ANCHOR: example
use std::collections::HashMap;

use petgraph::Direction::Incoming;
use petgraph::Direction::Outgoing;
use petgraph::algo::Cycle;
use petgraph::algo::toposort;
use petgraph::graph::DiGraph;
use petgraph::graph::NodeIndex;
use rayon::prelude::*;

/// Groups graph nodes by Kahn levels after verifying that the graph is acyclic.
///
/// # Errors
///
/// Returns the cycle reported by `petgraph` when the graph is not a DAG.
fn topological_levels<N, E>(
    graph: &DiGraph<N, E>,
) -> Result<Vec<Vec<NodeIndex>>, Cycle<NodeIndex>> {
    let _topological_order = toposort(graph, None)?;
    let mut indegrees = graph
        .node_indices()
        .map(|node| (node, graph.edges_directed(node, Incoming).count()))
        .collect::<HashMap<_, _>>();

    let mut ready = graph
        .node_indices()
        .filter(|node| indegrees.get(node).is_some_and(|degree| *degree == 0))
        .collect::<Vec<_>>();
    let mut levels = Vec::new();

    while !ready.is_empty() {
        ready.sort_unstable();
        let current_level = std::mem::take(&mut ready);
        let mut next_level = Vec::new();

        for node in current_level.iter().copied() {
            for successor in graph.neighbors_directed(node, Outgoing) {
                let indegree = indegrees
                    .get_mut(&successor)
                    .expect("every graph node has an indegree entry");
                *indegree -= 1;
                if *indegree == 0 {
                    next_level.push(successor);
                }
            }
        }

        levels.push(current_level);
        ready = next_level;
    }

    Ok(levels)
}

/// Calculates task totals in parallel for each ready DAG level.
///
/// # Errors
///
/// Returns an error if the task graph contains a cycle.
fn calculate_task_levels() -> Result<Vec<Vec<u32>>, Cycle<NodeIndex>> {
    let mut graph = DiGraph::<u32, ()>::new();
    let fetch = graph.add_node(2);
    let configure = graph.add_node(5);
    let compile = graph.add_node(3);
    let package = graph.add_node(7);
    graph.extend_with_edges([
        (fetch, compile),
        (configure, compile),
        (compile, package),
    ]);

    let levels = topological_levels(&graph)?;
    let mut totals = HashMap::with_capacity(graph.node_count());
    let mut values_by_level = Vec::with_capacity(levels.len());

    for level in levels {
        let values = level
            .par_iter()
            .map(|node| {
                let prerequisite_total = graph
                    .neighbors_directed(*node, Incoming)
                    .map(|prerequisite| {
                        totals
                            .get(&prerequisite)
                            .copied()
                            .expect("each prerequisite is evaluated in an earlier level")
                    })
                    .sum::<u32>();

                (*node, graph[*node] + prerequisite_total)
            })
            .collect::<Vec<_>>();

        values_by_level.push(values.iter().map(|(_, total)| *total).collect());
        totals.extend(values);
    }

    Ok(values_by_level)
}

/// Prints each level's computed task totals.
fn main() {
    let levels =
        calculate_task_levels().expect("the sample task graph is acyclic");
    for (index, totals) in levels.iter().enumerate() {
        println!("Level {} totals: {totals:?}", index + 1);
    }
}
// ANCHOR_END: example

/// Runs the example from the chapter's example runner.
pub fn run() {
    main();
}

/// Verifies that each ready level runs only after its prerequisites.
#[test]
fn test_topological_levels_wait_for_prerequisites() {
    let levels =
        calculate_task_levels().expect("the sample task graph is acyclic");
    assert_eq!(levels, vec![vec![2, 5], vec![10], vec![17]]);
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use petgraph::algo::toposort;
    use petgraph::graph::DiGraph;
    use petgraph::visit::EdgeRef;
    use proptest::prelude::*;
    use proptest::test_runner::TestRunner;

    /// Checks that every edge in a generated DAG points forward in the order.
    #[test]
    fn generated_dag_edges_follow_topological_order() {
        let mut runner = TestRunner::default();
        runner
            .run(
                &prop::collection::vec((0usize..8, 0usize..8), 0..40),
                |candidate_edges| {
                    let mut graph = DiGraph::<(), ()>::new();
                    let nodes =
                        (0..8).map(|_| graph.add_node(())).collect::<Vec<_>>();

                    for (source, target) in candidate_edges {
                        if source < target {
                            graph.add_edge(nodes[source], nodes[target], ());
                        }
                    }

                    let order = toposort(&graph, None)
                        .expect("forward-only edges form a DAG");
                    let positions = order
                        .iter()
                        .enumerate()
                        .map(|(position, node)| (*node, position))
                        .collect::<HashMap<_, _>>();

                    for edge in graph.edge_references() {
                        prop_assert!(
                            positions[&edge.source()]
                                < positions[&edge.target()]
                        );
                    }
                    Ok(())
                },
            )
            .expect("generated DAGs must respect topological ordering");
    }

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
