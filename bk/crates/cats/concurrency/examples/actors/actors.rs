// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A small actor built from a Tokio task and message channels.

use anyhow::Context;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

/// A command that the counter actor processes.
enum Message {
    /// Adds an amount and returns the updated total through the reply channel.
    Add {
        /// The amount to add to the actor's state.
        amount: u64,
        /// The one-use channel on which the actor sends the updated total.
        reply_to: oneshot::Sender<u64>,
    },
}

/// A cloneable handle for sending commands to the counter actor.
#[derive(Clone)]
struct CounterHandle {
    sender: mpsc::Sender<Message>,
}

impl CounterHandle {
    /// Adds an amount and waits for the actor's updated total.
    ///
    /// # Errors
    /// Returns an error if the actor has stopped before receiving or replying
    /// to the command.
    async fn add(&self, amount: u64) -> anyhow::Result<u64> {
        let (reply_to, reply_from) = oneshot::channel();
        self.sender
            .send(Message::Add { amount, reply_to })
            .await
            .context("counter actor stopped before receiving the command")?;
        reply_from
            .await
            .context("counter actor stopped before replying to the command")
    }
}

/// Starts the counter actor and returns its handle and task.
fn spawn_counter() -> (CounterHandle, tokio::task::JoinHandle<()>) {
    let (sender, mut receiver) = mpsc::channel(8);
    let task = tokio::spawn(async move {
        let mut total = 0;
        while let Some(Message::Add { amount, reply_to }) =
            receiver.recv().await
        {
            total += amount;
            // A dropped reply receiver means the caller no longer needs the
            // response.
            let _ = reply_to.send(total);
        }
    });

    (CounterHandle { sender }, task)
}

/// Runs the counter actor and prints the result of two commands.
///
/// # Errors
/// Returns an error if the actor task stops before replying or joining.
async fn run_actor() -> anyhow::Result<()> {
    let (counter, task) = spawn_counter();
    let first_total = counter.add(2).await?;
    let second_total = counter.add(3).await?;
    println!("Totals: {first_total}, {second_total}");

    drop(counter);
    task.await.context("counter actor task failed")?;
    Ok(())
}

/// Runs the asynchronous actor example on a Tokio runtime.
///
/// # Errors
/// Returns an error if the runtime cannot be created or the actor task fails.
fn main() -> anyhow::Result<()> {
    tokio::runtime::Runtime::new()?.block_on(run_actor())
}
// ANCHOR_END: example

/// Runs the Tokio actor example.
///
/// # Errors
/// Returns an error if the runtime cannot be created or the actor task fails.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use anyhow::Context;

    use super::spawn_counter;

    /// Verifies that cloned handles send commands to the same stateful actor.
    #[tokio::test]
    async fn cloned_handles_share_the_actors_state() -> anyhow::Result<()> {
        let (counter, task) = spawn_counter();
        let second_handle = counter.clone();

        assert_eq!(counter.add(2).await?, 2);
        assert_eq!(second_handle.add(3).await?, 5);

        drop(counter);
        drop(second_handle);
        task.await.context("counter actor task failed")?;
        Ok(())
    }
}
