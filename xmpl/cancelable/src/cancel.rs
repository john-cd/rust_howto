use crossbeam::channel;
use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;

use super::errors::TaskError;

const CANCEL_QUEUE_SIZE: usize = 10;

// Message.
#[derive(Debug)]
enum Cancel {
    All,
    Specific { id: u64 },
}

#[derive(Debug, Clone)]
pub struct CancelTokenSource {
    s_cancel: Sender<Cancel>,
    r_cancel: Receiver<Cancel>,
}

impl CancelTokenSource {
    pub fn new() -> Self {
        let (s_cancel, r_cancel) = channel::bounded::<Cancel>(CANCEL_QUEUE_SIZE);
        Self { s_cancel, r_cancel }
    }

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

    pub fn cancel_task(&self, id: u64) -> anyhow::Result<()> {
        self.cancel(Cancel::Specific { id })
    }

    fn cancel(&self, cancel: Cancel) -> anyhow::Result<()> {
        match self.s_cancel.try_send(cancel) {
            Ok(()) => tracing::info!("Cancel request sent."),
            Err(crossbeam_channel::TrySendError::Full(_)) => {
                tracing::error!("Cancellation channel is full.")
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

pub struct CancelToken {
    id: u64,
    r_cancel: Receiver<Cancel>,
}

impl CancelToken {
    pub fn check(&self) -> Result<(), TaskError> {
        match self.r_cancel.try_recv() {
            Ok(Cancel::All) => Err(TaskError::Interrupted),
            Ok(Cancel::Specific { id }) if id == self.id => Err(TaskError::Interrupted),
            _ => Ok(()),
        }
    }
}
