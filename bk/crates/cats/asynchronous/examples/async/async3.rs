// ANCHOR: example
/// Runs a task on the Tokio runtime and awaits its result.
#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async { 1 + 2 });
    let result = handle.await.expect("the spawned task should finish");

    assert_eq!(result, 3);
    println!("The spawned task returned {result}.");
}

// ANCHOR_END: example

/// Runs the example for the parent module.
pub fn run() {
    main();
}

/// Verifies that a spawned Tokio task returns its result through its join
/// handle.
#[test]
fn spawned_task_returns_its_result() {
    main();
}
