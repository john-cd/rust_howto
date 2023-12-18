use rayon::ThreadPoolBuildError;
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, error, event, info, trace, warn, Level};

pub struct SynchronousBackendManager {
    pool: rayon::ThreadPool,
    operations: HashMap<u32, Arc<dyn Any>>,
    next_id: u32,
}

impl SynchronousBackendManager {
    const N: usize = 0; // auto

    pub fn create() -> Result<Self, ThreadPoolBuildError> {
        // Build the threadpool
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(Self::N)
            .build()?;
        Ok(Self {
            pool,
            operations: HashMap::new(),
            next_id: 0,
        })
    }

    pub fn create_operation<I, O, OP>(&mut self, opr: OP) -> (u32, OperationHandle<I, O>)
    where
        OP: Fn(I, &dyn Responder<O>) + Send + 'static,     // &dyn Responder<O>
        I: Clone + Send + 'static,
        O: Clone + Send + 'static,
    {
        let operation_arc = Arc::new(Operation::new());
        // do stuff with operation before it gets consumed
        // Receiver does not implement clone; new Receiver handles are created by calling Sender::subscribe.
        let mut new_in_rx = operation_arc.in_tx.subscribe();
        // prepare a clone to be consumed by the pool thread
        let handle = OperationHandle::new(&operation_arc);
        let operation_arc_clone = operation_arc.clone();
        self.operations.insert(self.next_id, operation_arc);
        self.next_id += 1;

        // `install` executes the closure within the threadpool.
        // Any attempts to use join, scope, or parallel iterators will then operate within that threadpool.
        self.pool.spawn(move || {
            loop {
                match new_in_rx.blocking_recv() {
                    Ok(cmd) => opr(cmd, operation_arc_clone.as_ref()),
                    //  all Sender halves have dropped, indicating that no further values can be sent on the channel.
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(n)) => {
                        warn!("The receiver lagged too far behind. Next receive will return the oldest message still retained by the channel. {} skipped messages.", n);
                        continue;
                    },
                };
            }
        });
        (self.next_id - 1, handle)
    }

    // pub fn get_new_handle_for(&self, id: u32) -> OperationHandle<I, O> {
    //     self.operations.get(&id).map( |v| OperationHandle::new(v))
    // }
}

use tokio::sync::broadcast::error::RecvError;
use tokio::sync::broadcast::{self, Receiver, Sender};

// wraps in / out queues for a given operation
// I = inputs to the synchronous operation
// O = outputs of the synchronous operation
struct Operation<I, O> {
    in_tx: Sender<I>,
    out_tx: Sender<O>,
}

impl<I, O> Operation<I, O>
where
    I: Clone,
    O: Clone,
{
    fn new() -> Self {
        let (in_tx, _) = broadcast::channel(16);
        let (out_tx, _) = broadcast::channel(16);
        Self { in_tx, out_tx }
    }
}

// trait passed to the synchronous operation for it to respond to the handler
pub trait Responder<O>: Sync + Send {
    fn respond_with(&self, out: O);
}

impl<I: Send, O: Send> Responder<O> for Operation<I, O> {
    fn respond_with(&self, resp: O) {
        // A send operation can only fail if there are no active receivers, implying that the message could never be received.
        self.out_tx.send(resp).unwrap_or_else(|error| {
            //info!("There are no active receivers: {:?}", error);
            0
        });
    }
}

pub struct OperationHandle<I, O> {
    operation: Arc<Operation<I, O>>, // smart pointer to the Operation
    out_rx: broadcast::Receiver<O>,
}

impl<I, O> OperationHandle<I, O>
where
    I: Clone,
    O: Clone,
{
    fn new(operation: &Arc<Operation<I, O>>) -> OperationHandle<I, O> {
        Self {
            operation: operation.clone(),
            out_rx: operation.out_tx.subscribe(),
        }
    }

    pub fn send_command(&self, cmd: I) {
        self.operation.in_tx.send(cmd).unwrap_or_else(|err| {
            //info!("There are no active receivers: {:?}", err);
            0
        });
    }

    pub fn get_response_blocking(&mut self) -> Option<O> {
        match self.out_rx.blocking_recv() {
            Ok(out_msg) => Some(out_msg),
            //  all Sender have dropped, indicating that no further values can be sent on the channel.
            Err(RecvError::Closed) => None,
            Err(RecvError::Lagged(n)) => {
                warn!("The receiver lagged too far behind. Next receive will return the oldest message still retained by the channel. {} skipped messages.", n);
                match self.out_rx.blocking_recv() {
                    Ok(out_msg) => Some(out_msg),
                    Err(RecvError::Closed) => None,
                    _ => panic!("Retrieved `RecvError::Lagged` twice"),
                }
            }
        }
    }

    pub async fn get_response(&mut self) -> Option<O> {
        match self.out_rx.recv().await {
            Ok(out_msg) => Some(out_msg),
            //  all Sender have dropped, indicating that no further values can be sent on the channel.
            Err(RecvError::Closed) => None,
            Err(RecvError::Lagged(n)) => {
                warn!("The receiver lagged too far behind. Next receive will return the oldest message still retained by the channel. {} skipped messages.", n);
                match self.out_rx.recv().await {
                    Ok(out_msg) => Some(out_msg),
                    Err(RecvError::Closed) => None,
                    _ => panic!("Retrieved `RecvError::Lagged` twice"),
                }
            }
        }
    }
}

// Cloning creates a new Handle that can be used on another thread and, within it, a new receiver
impl<I, O> Clone for OperationHandle<I, O> {
    fn clone(&self) -> Self {
        let receiver = self.operation.out_tx.subscribe();
        Self {
            operation: self.operation.clone(),
            out_rx: receiver,
        }
    }
}
