//! Moves eligible unicast messages into the shared-memory handoff path.
//!
//! TODO Reviewed ??-??-2026.

// Copyright (c) 2026 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
//
use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;

use flume::Sender;
use flume::TryRecvError;
use static_init::dynamic;
use zenoh_buffers::reader::DidntRead;
use zenoh_buffers::reader::Reader;
use zenoh_buffers::writer::DidntWrite;
use zenoh_buffers::writer::Writer;
use zenoh_codec::RCodec;
use zenoh_codec::WCodec;
use zenoh_codec::Zenoh080;
use zenoh_core::zlock;
use zenoh_protocol::core::Priority;
use zenoh_protocol::core::Reliability;
use zenoh_result::ZResult;
use zenoh_runtime::ZRuntime;
use zenoh_shm::ShmBufHardRef;

use crate::common::shm::handoff::PriorityContainer;
use crate::unicast::establishment::ext::shm::auth::AuthUnicast;
use crate::unicast::establishment::ext::shm::segment::RXAuthSegment;
use crate::unicast::establishment::ext::shm::segment::ShmCounterID;
use crate::unicast::establishment::ext::shm::segment::ShmRXCounterLease;
use crate::unicast::establishment::ext::shm::segment::ShmTXCounterLease;

/// Defines the possible handoff config cases handled by this code.
#[derive(Debug)]
pub enum HandoffConfig<T: Sized> {
    /// Represents the disabled case of handoff config.
    Disabled,
    /// Represents the per prio case of handoff config.
    PerPrio(
        /// Per-priority queues used to schedule handoff transactions.
        PriorityContainer<T>,
    ),
}

impl<T: Sized + Clone> Clone for HandoffConfig<T> {
    /// Handles the clone behavior for this component.
    fn clone(&self) -> Self {
        match self {
            Self::Disabled => Self::Disabled,
            Self::PerPrio(arg0) => Self::PerPrio(arg0.clone()),
        }
    }
}

impl<T: Sized> HandoffConfig<T> {
    /// Constructs this value from a fn.
    pub fn from_fn(
        reliability: Reliability,
        ctor_fn: impl Fn(Priority) -> ZResult<T>,
    ) -> ZResult<Self> {
        Ok(match reliability {
            Reliability::BestEffort => Self::Disabled,
            Reliability::Reliable => Self::PerPrio(PriorityContainer::from_fn(ctor_fn)?),
        })
    }
}

/// Alias for `HandoffConfig<ShmRXCounterLease>` used to represent rx handoff channel.
pub type RxHandoffChannel = HandoffConfig<ShmRXCounterLease>;

impl RxHandoffChannel {
    /// Handles the new rx behavior for this component.
    pub fn new_rx(segment: &Arc<RXAuthSegment>, ids: HandoffCounterIds) -> ZResult<Self> {
        Ok(match ids {
            HandoffCounterIds::Disabled => Self::Disabled,
            HandoffCounterIds::PerPrio(prio_container) => {
                let prio_container = prio_container.map_fallable(|counter_id| {
                    ShmRXCounterLease::new(segment.clone(), *counter_id)
                })?;
                Self::PerPrio(prio_container)
            }
        })
    }

    /// Handles the on rx behavior for this component.
    pub fn on_rx(&self, priority: Priority) {
        match self {
            HandoffConfig::PerPrio(prio_container) => prio_container[priority].counter_decrease(),
            HandoffConfig::Disabled => {}
        }
    }
}

/// Data for tx handoff channel operations, including handoff config and handoff counter ids.
#[derive(Debug)]
pub struct TxHandoffChannel(
    /// Shared-memory handoff configuration used by the transmitting channel.
    HandoffConfig<ShmTXCounterLease>,
);
impl TxHandoffChannel {
    /// Creates a disabled instance that does not accept transport work.
    pub fn new_disabled() -> Self {
        Self(HandoffConfig::Disabled)
    }

    /// Handles the new tx behavior for this component.
    pub fn new_tx(reliability: Reliability, auth: &AuthUnicast) -> ZResult<Self> {
        let ctor_fn = |_prio| auth.lease_tx_counter();
        Ok(Self(HandoffConfig::from_fn(reliability, ctor_fn)?))
    }

    /// Handles the ids behavior for this component.
    pub fn ids(&self) -> HandoffCounterIds {
        match &self.0 {
            HandoffConfig::PerPrio(prio_container) => {
                let prio_container = prio_container.map_ref(|counter| counter.id());
                HandoffCounterIds::PerPrio(prio_container)
            }
            HandoffConfig::Disabled => HandoffCounterIds::Disabled,
        }
    }
}

/// Stores shared global handoff reactor state used by this module.
#[dynamic(lazy, drop)]
static mut GLOBAL_HANDOFF_REACTOR: HandoffReactor = HandoffReactor::new();

/// Data for handoff reactor operations, including task sender.
struct HandoffReactor {
    /// Channel sender used to hand work to the background task.
    task_sender: Sender<Arc<TxHandoffTask>>,
}

impl HandoffReactor {
    /// Creates a new handoff reactor with the supplied configuration.
    fn new() -> Self {
        let (task_sender, task_receiver) = flume::unbounded();

        ZRuntime::Net.spawn(async move {
            // we mostly iterate over this rather than add\remove elements, so Vec should be optimal
            // choice
            let mut tasks: Vec<Arc<TxHandoffTask>> = Vec::default();
            loop {
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                loop {
                    match task_receiver.try_recv() {
                        Ok(new_task) => tasks.push(new_task),
                        Err(TryRecvError::Empty) => break,
                        Err(TryRecvError::Disconnected) => return,
                    }
                }
                tasks.retain(|t| {
                    t.poll();
                    Arc::strong_count(t) > 1
                });
            }
        });

        Self { task_sender }
    }
}

/// Data for tx handoff task operations, including counter and handoffs len.
#[derive(Debug)]
struct TxHandoffTask {
    /// Counter tracking the number of completed operations.
    counter: ShmTXCounterLease,
    /// handoffs len value tracked by this type.
    handoffs_len: AtomicUsize,
    /// In-flight handoffs that still need to complete.
    handoffs: lockfree::queue::Queue<Option<ShmBufHardRef>>,
}

impl TxHandoffTask {
    /// Creates a new tx handoff task with the supplied configuration.
    fn new(counter: ShmTXCounterLease) -> Self {
        Self {
            counter,
            handoffs: Default::default(),
            handoffs_len: AtomicUsize::new(0),
        }
    }

    /// Handles the poll behavior for this component.
    fn poll(&self) {
        let to_pop = self.handoffs_len.load(SeqCst) as isize - self.counter.counter() as isize;
        if to_pop > 0 {
            for _ in 0..to_pop {
                self.handoffs.pop();
            }
            self.handoffs_len.fetch_sub(to_pop as usize, SeqCst);
        }
    }
}

/// Data for tx handoff inner operations, including task and not commit.
#[derive(Debug)]
pub struct TxHandoffInner {
    /// Background task responsible for this connection lifecycle step.
    task: Arc<TxHandoffTask>,
    /// not commit value tracked by this type.
    not_commit: Mutex<VecDeque<ShmBufHardRef>>,
}

impl TxHandoffInner {
    /// Creates a new tx handoff inner with the supplied configuration.
    pub fn new(counter: ShmTXCounterLease) -> Self {
        let task = Arc::new(TxHandoffTask::new(counter));

        GLOBAL_HANDOFF_REACTOR
            .read()
            .task_sender
            .send(task.clone())
            .unwrap();

        Self {
            task,
            not_commit: Default::default(),
        }
    }
}

/// Data for tx handoff operations, including inner.
#[derive(Clone, Debug)]
pub struct TxHandoff {
    /// Wrapped value that owns the underlying state.
    inner: Arc<TxHandoffInner>,
}

impl TxHandoff {
    /// Creates a new tx handoff with the supplied configuration.
    pub fn new(counter: ShmTXCounterLease) -> Self {
        let inner = Arc::new(TxHandoffInner::new(counter));
        Self { inner }
    }

    /// Handles the lock behavior for this component.
    fn lock(&self) -> LockedTxHandoff {
        let inner = self.inner.clone();
        let lock = zlock!(self.inner.not_commit);

        // SAFETY: `inner` keeps the mutex alive. LockedTxHandoff drops the guard first.
        let lock: std::sync::MutexGuard<'static, VecDeque<ShmBufHardRef>> =
            unsafe { std::mem::transmute(lock) };
        LockedTxHandoff { inner, lock }
    }
}

/// Data for locked tx handoff operations, including lock and inner.
struct LockedTxHandoff {
    // SAFETY: `lock` borrows into the mutex that `inner` owns. Rust drops fields in
    // declaration order. Keep `lock` first, or `inner` frees the mutex too early.
    lock: std::sync::MutexGuard<'static, VecDeque<ShmBufHardRef>>,
    /// Wrapped value that owns the underlying state.
    inner: Arc<TxHandoffInner>,
}

impl LockedTxHandoff {
    /// Handles the push behavior for this component.
    pub fn push(&mut self, reference: ShmBufHardRef) {
        self.inner.task.counter.add(1);
        self.lock.push_back(reference);
    }

    /// Handles the cancel behavior for this component.
    pub fn cancel(mut self) {
        self.inner.task.counter.sub(self.lock.len() as u32);
        self.lock.clear();
    }

    /// Handles the commit behavior for this component.
    pub fn commit(mut self) {
        let len = self.lock.len();
        while let Some(h) = self.lock.pop_front() {
            self.inner.task.handoffs.push(Some(h));
        }
        self.inner.task.handoffs_len.fetch_add(len, SeqCst);
    }
}

/// Data for tx handoff storage operations, including handoff config and handoff config.
#[derive(Debug, Clone)]
pub struct TxHandoffStorage(
    /// Handoff configuration that owns the transmit-side storage.
    HandoffConfig<TxHandoff>,
);
impl TxHandoffStorage {
    /// Creates a disabled instance that does not accept transport work.
    pub fn new_disabled() -> Self {
        Self(HandoffConfig::Disabled)
    }

    /// Returns the value requested from this object.
    fn get(&self, priority: Priority) -> Option<&TxHandoff> {
        match &self.0 {
            HandoffConfig::PerPrio(prio_container) => Some(&prio_container[priority]),
            HandoffConfig::Disabled => None,
        }
    }
}

impl From<TxHandoffChannel> for TxHandoffStorage {
    /// Constructs this type from the supplied source value.
    fn from(value: TxHandoffChannel) -> Self {
        match value.0 {
            HandoffConfig::PerPrio(prio_container) => {
                let prio_container = prio_container.map(TxHandoff::new);
                Self(HandoffConfig::PerPrio(prio_container))
            }
            HandoffConfig::Disabled => Self(HandoffConfig::Disabled),
        }
    }
}

/// Data for lazy transaction operations, including storage and priority.
struct LazyTransaction<'a> {
    /// Storage backing the event bus or record collection.
    storage: &'a TxHandoffStorage,
    /// Transport priority assigned to this channel.
    priority: Priority,
}

impl<'a> LazyTransaction<'a> {
    /// Creates a new lazy transaction with the supplied configuration.
    fn new(storage: &'a TxHandoffStorage, priority: Priority) -> Self {
        Self { storage, priority }
    }
}

/// Defines the possible open transaction state cases handled by this code.
enum OpenTransactionState<'a> {
    /// Represents the lazy case of open transaction state.
    Lazy(
        /// Transaction that has not acquired the transmit lock yet.
        LazyTransaction<'a>,
    ),
    /// Represents the disabled case of open transaction state.
    Disabled,
    /// Represents the init case of open transaction state.
    Init(
        /// Handoff state with its transmit lock already acquired.
        LockedTxHandoff,
    ),
}

impl<'a> OpenTransactionState<'a> {
    /// Attempts to access and reports whether the operation succeeds.
    fn try_access(&mut self) -> Option<&mut LockedTxHandoff> {
        match self {
            OpenTransactionState::Lazy(lazy_transaction) => {
                *self = match lazy_transaction.storage.get(lazy_transaction.priority) {
                    Some(handoff) => Self::Init(handoff.lock()),
                    None => Self::Disabled,
                };
                self.try_access()
            }
            OpenTransactionState::Disabled => None,
            OpenTransactionState::Init(locked_tx_handoff) => Some(locked_tx_handoff),
        }
    }

    /// Handles the on tx behavior for this component.
    fn on_tx(&mut self, reference: ShmBufHardRef) {
        if let Some(handoff) = self.try_access() {
            handoff.push(reference);
        }
    }

    /// Closes the transport resource and releases its associated state.
    fn close(self) -> ClosedTransactionState {
        if let OpenTransactionState::Init(locked) = self {
            return ClosedTransactionState::new(locked);
        }
        ClosedTransactionState::new_disabled()
    }
}

/// State for closed transaction state operations that tracks state.
struct ClosedTransactionState {
    /// Protocol state that determines which handshake transition is valid.
    state: Option<LockedTxHandoff>,
}

impl ClosedTransactionState {
    /// Creates a new closed transaction state with the supplied configuration.
    fn new(state: LockedTxHandoff) -> Self {
        Self { state: Some(state) }
    }

    /// Creates a disabled instance that does not accept transport work.
    const fn new_disabled() -> Self {
        Self { state: None }
    }

    /// Handles the commit behavior for this component.
    fn commit(self) {
        if let Some(state) = self.state {
            state.commit();
        }
    }

    /// Handles the cancel behavior for this component.
    fn cancel(self) {
        if let Some(state) = self.state {
            state.cancel();
        }
    }
}

/// Data for open transaction operations, including state.
pub struct OpenTransaction<'a> {
    /// Protocol state that determines which handshake transition is valid.
    state: OpenTransactionState<'a>,
}

impl<'a> OpenTransaction<'a> {
    /// Creates a new open transaction with the supplied configuration.
    pub fn new(storage: &'a TxHandoffStorage, priority: Priority) -> Self {
        let lazy = LazyTransaction::new(storage, priority);
        Self {
            state: OpenTransactionState::Lazy(lazy),
        }
    }

    /// Creates a disabled instance that does not accept transport work.
    pub const fn new_disabled() -> Self {
        Self {
            state: OpenTransactionState::Disabled,
        }
    }

    /// Handles the on tx behavior for this component.
    pub fn on_tx(&mut self, reference: ShmBufHardRef) {
        self.state.on_tx(reference);
    }

    /// Closes the transport resource and releases its associated state.
    pub fn close(self) -> TxHandoffTransaction {
        let closed = self.state.close();
        TxHandoffTransaction::new(closed)
    }
}

/// Data for tx handoff transaction operations, including state.
pub struct TxHandoffTransaction {
    /// Protocol state that determines which handshake transition is valid.
    state: ClosedTransactionState,
}

impl TxHandoffTransaction {
    /// Creates a new tx handoff transaction with the supplied configuration.
    fn new(state: ClosedTransactionState) -> Self {
        Self { state }
    }

    /// Handles the commit behavior for this component.
    pub fn commit(&mut self) {
        let mut state = ClosedTransactionState::new_disabled();
        std::mem::swap(&mut self.state, &mut state);
        state.commit();
    }
}

impl Drop for TxHandoffTransaction {
    /// Releases the resources owned by this value.
    fn drop(&mut self) {
        let mut state = ClosedTransactionState::new_disabled();
        std::mem::swap(&mut self.state, &mut state);
        state.cancel();
    }
}

impl<W> WCodec<&PriorityContainer<ShmCounterID>, &mut W> for Zenoh080
where
    W: Writer,
{
    /// Alias for `Result<(), DidntWrite>` used to represent output.
    type Output = Result<(), DidntWrite>;

    /// Writes the supplied value to the configured output sink.
    fn write(self, writer: &mut W, x: &PriorityContainer<ShmCounterID>) -> Self::Output {
        for obj in x {
            self.write(&mut *writer, obj)?;
        }
        Ok(())
    }
}

impl<R> RCodec<PriorityContainer<ShmCounterID>, &mut R> for Zenoh080
where
    R: Reader,
{
    /// Alias for `DidntRead` used to represent error.
    type Error = DidntRead;

    /// Reads the next value from the supplied input source.
    fn read(self, reader: &mut R) -> Result<PriorityContainer<ShmCounterID>, Self::Error> {
        PriorityContainer::from_fn(|_| self.read(&mut *reader))
    }
}

/// Alias for `HandoffConfig<ShmCounterID>` used to represent handoff counter ids.
pub type HandoffCounterIds = HandoffConfig<ShmCounterID>;

impl<W> WCodec<&HandoffCounterIds, &mut W> for Zenoh080
where
    W: Writer,
{
    /// Alias for `Result<(), DidntWrite>` used to represent output.
    type Output = Result<(), DidntWrite>;

    /// Writes the supplied value to the configured output sink.
    fn write(self, writer: &mut W, x: &HandoffCounterIds) -> Self::Output {
        match x {
            HandoffConfig::Disabled => {
                self.write(&mut *writer, &0u8)?;
            }
            HandoffConfig::PerPrio(prio_container) => {
                self.write(&mut *writer, &1u8)?;
                self.write(&mut *writer, prio_container)?;
            }
        }
        Ok(())
    }
}

impl<R> RCodec<HandoffCounterIds, &mut R> for Zenoh080
where
    R: Reader,
{
    /// Alias for `DidntRead` used to represent error.
    type Error = DidntRead;

    /// Reads the next value from the supplied input source.
    fn read(self, reader: &mut R) -> Result<HandoffCounterIds, Self::Error> {
        let status: u8 = self.read(&mut *reader)?;
        Ok(if status == 0 {
            HandoffConfig::Disabled
        } else {
            let prio: PriorityContainer<ShmCounterID> = self.read(&mut *reader)?;
            HandoffCounterIds::PerPrio(prio)
        })
    }
}

/// Provides race tests behavior for the handoff component.
#[cfg(test)]
mod race_tests {
    use std::sync::atomic::Ordering::SeqCst;
    use std::thread;
    use std::time::Duration;

    use zenoh_core::Wait;
    use zenoh_shm::ShmBufHardRef;
    use zenoh_shm::api::protocol_implementations::posix::posix_shm_provider_backend_binary_heap::PosixShmProviderBackendBinaryHeap;
    use zenoh_shm::api::provider::shm_provider::ShmProviderBuilder;

    use super::*;
    use crate::unicast::establishment::ext::shm::segment::TXAuthSegment;

    /// Checks that concurrent same priority transactions are isolated.
    #[test]
    fn concurrent_same_priority_transactions_are_isolated() {
        let backend = PosixShmProviderBackendBinaryHeap::builder(4096)
            .wait()
            .unwrap();
        let shm_provider = ShmProviderBuilder::backend(backend).wait();

        let buf_a = shm_provider.alloc(64).wait().unwrap();
        let buf_b = shm_provider.alloc(64).wait().unwrap();
        let ref_a = ShmBufHardRef::from(&buf_a);
        let ref_b = ShmBufHardRef::from(&buf_b);

        let auth_segment = Arc::new(TXAuthSegment::create(0, &[]).unwrap());
        let lease = ShmTXCounterLease::new(auth_segment).unwrap();
        let handoff = TxHandoff::new(lease);

        let h_a = handoff.clone();
        let h_b = handoff.clone();

        let a = thread::spawn(move || {
            let mut locked = h_a.lock();
            locked.push(ref_a);
            std::thread::sleep(Duration::from_millis(1000));
            // Simulate A's own `pipeline.push_network_message` failing: the transaction is
            // dropped without ever calling `commit()`, so only `cancel()` runs.
            locked.cancel();
        });

        let b = thread::spawn(move || {
            let mut locked = h_b.lock();
            locked.push(ref_b);
            std::thread::sleep(Duration::from_millis(500));
            locked.commit();
        });

        a.join().unwrap();
        b.join().unwrap();

        // What *should* hold if same-priority transactions were isolated: only B's entry was
        // ever committed, and A's cancel rolled back exactly A's own increment.
        //
        // What actually happens: B's `commit()` drains the shared `not_commit` queue wholesale,
        // taking A's still-staged entry with it. A's later `cancel()` finds nothing to roll
        // back, so A's counter increment leaks and A's hard ref is stuck in the handoff queue
        // permanently (no RX ack for a message that was never sent).
        let handoffs_len = handoff.inner.task.handoffs_len.load(SeqCst);
        let counter = handoff.inner.task.counter.counter();

        assert_eq!(
            handoffs_len, 1,
            "expected only B's transaction to be committed (isolated staging); \
             got {handoffs_len} committed entries — A's uncommitted push leaked into B's commit, \
             confirming `internal_schedule` does NOT serialize concurrent same-priority sends"
        );
        assert_eq!(
            counter, 1,
            "expected the counter to reflect only B's committed hard ref after A's cancel \
             rolled back its own increment; got {counter} — A's increment leaked"
        );
    }
}
