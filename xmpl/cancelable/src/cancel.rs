//! Cancelling tasks with tokens that are checked while the task runs.
//!
//! TODO Reviewed ??-??-2026

use crossbeam::channel;
use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;

use super::errors::TaskError;

/// Maximum number of cancel requests that can be queued.
const CANCEL_QUEUE_SIZE: usize = 10;

// Message.
/// A request to cancel tasks.
#[derive(Debug)]
enum Cancel {
    /// Cancel all tasks.
    All,
    /// Cancel one task.
    Specific {
        /// Identifier of the task to cancel.
        id: u64,
    },
}

/// Sends cancel requests and creates the tokens that receive them.
#[derive(Debug, Clone)]
pub struct CancelTokenSource {
    /// Sends the cancel requests.
    s_cancel: Sender<Cancel>,
    /// Receives the cancel requests; cloned into each token.
    r_cancel: Receiver<Cancel>,
}

impl CancelTokenSource {
    /// Creates a source with an empty queue of requests.
    pub fn new() -> Self {
        let (s_cancel, r_cancel) = channel::bounded::<Cancel>(CANCEL_QUEUE_SIZE);
        Self { s_cancel, r_cancel }
    }

    /// Creates the token of the task `id`.
    pub fn make_token(&self, id: u64) -> CancelToken {
        CancelToken {
            id,
            r_cancel: self.r_cancel.clone(),
        }
    }

    /// Request all token to be interrupted.
    pub fn cancel_all(&self) -> anyhow::Result<()> {
        self.cancel(Cancel::All)
    }

    /// Requests that the task `id` is interrupted.
    pub fn cancel_task(&self, id: u64) -> anyhow::Result<()> {
        self.cancel(Cancel::Specific { id })
    }

    /// Sends a cancel request without blocking.
    fn cancel(&self, cancel: Cancel) -> anyhow::Result<()> {
        match self.s_cancel.try_send(cancel) {
            Ok(()) => tracing::info!("Cancel request sent."),
            Err(crossbeam_channel::TrySendError::Full(_)) => {
                tracing::error!("Cancellation channel is full.");
            }
            Err(e) => {
                tracing::error!("Error when trying to cancel: {e}");
                return Err(e.into());
            }
        }
        Ok(())
    }
}

impl Default for CancelTokenSource {
    fn default() -> Self {
        Self::new()
    }
}

/// A token that a task checks to know whether it should stop.
pub struct CancelToken {
    /// Identifier of the task.
    id: u64,
    /// Receives the cancel requests.
    r_cancel: Receiver<Cancel>,
}

impl CancelToken {
    /// Returns an error if the task has been asked to stop.
    pub fn check(&self) -> Result<(), TaskError> {
        match self.r_cancel.try_recv() {
            Ok(Cancel::All) => Err(TaskError::Interrupted),
            Ok(Cancel::Specific { id }) if id == self.id => Err(TaskError::Interrupted),
            _ => Ok(()),
        }
    }
}
