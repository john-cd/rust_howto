// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Cancels a group of asynchronous tasks with `CancellationToken`.

use tokio_util::sync::CancellationToken;

/// Cancels a child task when its parent token is cancelled.
async fn cancel_child_task() -> anyhow::Result<bool> {
    let parent = CancellationToken::new();
    let child = parent.child_token();
    let worker = tokio::spawn(async move {
        child.cancelled().await;
    });

    parent.cancel();
    worker.await?;
    Ok(parent.is_cancelled())
}

/// Runs the cancellation-token example's behavior.
fn run_example() -> anyhow::Result<()> {
    let runtime = tokio::runtime::Builder::new_current_thread().build()?;
    let cancelled = runtime.block_on(cancel_child_task())?;
    println!("Worker cancelled: {cancelled}");
    Ok(())
}

/// Runs the cancellation-token example.
fn main() -> anyhow::Result<()> {
    run_example()
}

// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    run_example()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies cancelling a parent token wakes a child task.
    #[test]
    fn cancelling_parent_wakes_child_task() -> anyhow::Result<()> {
        let runtime = tokio::runtime::Builder::new_current_thread().build()?;

        assert!(runtime.block_on(cancel_child_task())?);
        Ok(())
    }
}
