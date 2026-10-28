// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Topological sorting of a Directed Acyclic Graph (DAG) using `petgraph`.
//!
//! ### What is Topological Sorting?
//! Topological sorting produces a linear sequence of graph nodes such that for
//! every directed edge $u \to v$, node $u$ appears before node $v$ in the
//! sequence.
//!
//! ### Directed Acyclic Graph (DAG) Requirement
//! Topological sorting is **only** defined for Directed Acyclic Graphs (DAGs).
//! If a graph contains a directed cycle ($A \to B \to C \to A$), no valid
//! linear dependency ordering exists, and `toposort` returns a cycle detection
//! error.
//!
//! ### Purpose & Use Cases
//! - **Build Systems & Compilers**: Resolving compilation order for source
//!   modules and crate dependencies (e.g., Cargo build graph).
//! - **Task Schedulers**: Determining valid execution sequences for data
//!   workflow pipelines (e.g., ETL jobs or Airflow DAGs).
//! - **Instruction Scheduling**: Ordering hardware instructions to avoid data
//!   hazard stalls.

use petgraph::Graph;
use petgraph::algo::toposort;

/// Demonstrates topological sorting for dependency resolution in a task graph.
fn main() {
    // Create a DAG representing task dependencies.
    // A directed edge `A -> B` signifies "Task A must complete before Task B
    // can start".
    let mut graph = Graph::<&str, ()>::new();

    // Add tasks as graph nodes.
    let fetch_data = graph.add_node("Fetch Data");
    let validate = graph.add_node("Validate");
    let transform = graph.add_node("Transform");
    let load = graph.add_node("Load");
    let report = graph.add_node("Report");

    // Add directed dependency edges.
    graph.add_edge(fetch_data, validate, ());
    graph.add_edge(validate, transform, ());
    graph.add_edge(transform, load, ());
    graph.add_edge(load, report, ());
    graph.add_edge(validate, report, ()); // Validation outputs feed directly into Report.

    // Execute topological sort over the DAG.
    // Returns `Err(Cycle)` if any circular dependency loop is encountered.
    let order = toposort(&graph, None).expect("the example graph is acyclic.");
    println!("Topological order (tasks to execute in sequence):");
    for node in &order {
        println!("  - {}", graph[*node]);
    }

    // Verify ordering constraints:
    // "Fetch Data" must precede "Validate", and "Validate" must precede "Load".
    let positions: Vec<&str> = order.iter().map(|node| graph[*node]).collect();
    let fetch_position = positions
        .iter()
        .position(|&task| task == "Fetch Data")
        .expect("Fetch Data is present in the example graph.");
    let validate_position = positions
        .iter()
        .position(|&task| task == "Validate")
        .expect("Validate is present in the example graph.");
    let load_position = positions
        .iter()
        .position(|&task| task == "Load")
        .expect("Load is present in the example graph.");

    assert!(fetch_position < validate_position);
    assert!(validate_position < load_position);
}

// ANCHOR_END: example

/// Runs the `petgraph` topological sort example by calling `main()`.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_petgraph_toposort_example_entry_point() {
        main();
    }
}
