//! Pin a borrowed future, observe its cancellation, and own spawned tasks.

use std::future::pending;

use tokio::sync::oneshot;

struct DropNotice(Option<oneshot::Sender<()>>);

impl Drop for DropNotice {
    fn drop(&mut self) {
        if let Some(sender) = self.0.take() {
            let _ = sender.send(());
        }
    }
}

// ANCHOR: example
async fn wait_forever(on_drop: oneshot::Sender<()>) {
    let _notice = DropNotice(Some(on_drop));
    pending::<()>().await;
}

async fn cancel_borrowed_future() -> Result<(), oneshot::error::RecvError> {
    let (sender, dropped) = oneshot::channel();
    {
        let future = wait_forever(sender);
        tokio::pin!(future);
        tokio::select! {
            () = &mut future => unreachable!("the future never completes"),
            () = tokio::task::yield_now() => {}
        }
        // Dropping the future at the end of this scope cancels its pending work.
    }
    dropped.await
}

async fn own_spawned_task() {
    let completed = tokio::spawn(async { 7_u8 });
    assert_eq!(completed.await.expect("task completed"), 7);

    let pending = tokio::spawn(async { pending::<()>().await });
    pending.abort();
    assert!(pending.await.expect_err("task was aborted").is_cancelled());
}
// ANCHOR_END: example

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = tokio::runtime::Builder::new_current_thread().build()?;
    runtime.block_on(async {
        cancel_borrowed_future().await?;
        own_spawned_task().await;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn cancellation_drops_the_future() {
        cancel_borrowed_future().await.expect("drop notice arrived");
    }

    #[tokio::test]
    async fn spawned_tasks_are_joined_or_aborted() {
        own_spawned_task().await;
    }
}
