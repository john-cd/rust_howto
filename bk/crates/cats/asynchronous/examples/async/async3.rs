// ANCHOR: example
//! Run a task on the Tokio runtime and await its result.
//!
//! `tokio::spawn` starts a task that runs concurrently, possibly on another
//! thread, and returns a handle. Awaiting the handle gives the task's result,
//! or an error if it panicked. Tasks are cheap, so a program can spawn
//! thousands. `#[tokio::main]` creates the runtime that runs them.

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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawned_task_returns_its_result() {
        main();
    }

    #[tokio::test]
    async fn join_error_is_reported_when_the_task_panics() {
        let handle = tokio::spawn(async {
            panic!("boom");
        });
        let err = handle.await.expect_err("the task panicked");
        assert!(err.is_panic());
    }

    #[tokio::test]
    async fn aborted_task_is_reported_as_cancelled() {
        let handle = tokio::spawn(std::future::pending::<()>());
        handle.abort();
        let err = handle.await.expect_err("the task was aborted");
        assert!(err.is_cancelled());
    }
}
