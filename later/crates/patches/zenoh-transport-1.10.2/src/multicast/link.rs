//! Represents a multicast link and exposes its transport read and write operations.
//!
//! TODO Reviewed ??-??-2026.

// Copyright (c) 2023 ZettaScale Technology
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
//!
//

use std::convert::TryInto;
use std::fmt;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;

use tokio::task::JoinHandle;
use zenoh_buffers::BBuf;
use zenoh_buffers::ZSlice;
use zenoh_buffers::ZSliceBuffer;
use zenoh_core::zcondfeat;
use zenoh_core::zlock;
use zenoh_link::LinkMulticast;
use zenoh_link::Locator;
use zenoh_protocol::core::Bits;
use zenoh_protocol::core::Priority;
use zenoh_protocol::core::Resolution;
use zenoh_protocol::core::WhatAmI;
use zenoh_protocol::core::ZenohIdProto;
use zenoh_protocol::transport::BatchSize;
use zenoh_protocol::transport::Close;
use zenoh_protocol::transport::Join;
use zenoh_protocol::transport::PrioritySn;
use zenoh_protocol::transport::TransportMessage;
use zenoh_protocol::transport::TransportSn;
use zenoh_protocol::transport::join::ext::PatchType;
use zenoh_result::ZResult;
use zenoh_result::zerror;
use zenoh_sync::RecyclingObject;
use zenoh_sync::RecyclingObjectPool;
use zenoh_sync::Signal;

use crate::common::batch::BatchConfig;
use crate::common::batch::Encode;
use crate::common::batch::Finalize;
use crate::common::batch::RBatch;
use crate::common::batch::WBatch;
use crate::common::pipeline::PipelineConsumer;
use crate::common::pipeline::TransmissionPipeline;
use crate::common::pipeline::TransmissionPipelineConf;
use crate::common::pipeline::TransmissionPipelineConsumer;
use crate::common::pipeline::TransmissionPipelineProducer;
use crate::common::priority::TransportPriorityTx;
use crate::multicast::transport::TransportMulticastInner;

/// *************************
// TRANSPORT MULTICAST LINK
/// *************************
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct TransportLinkMulticastConfig {
    /// Batching configuration for this transport.
    pub(crate) batch: BatchConfig,
}
/// Multicast transport link and its send and receive state.

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct TransportLinkMulticast {
    /// Transport link associated with this state.
    pub(crate) link: LinkMulticast,
    /// Settings that control how this transport link multicast is constructed or processed.
    pub(crate) config: TransportLinkMulticastConfig,
}

impl TransportLinkMulticast {
    /// Constructs a usable initial transport link multicast value with the defaults required by the
    /// link component.
    ///
    /// # Arguments
    /// * `link` - Transport link to add, remove, or use.
    /// * `config` - Configuration applied to this operation.
    pub(crate) fn new(link: LinkMulticast, mut config: TransportLinkMulticastConfig) -> Self {
        config.batch.mtu = link.get_mtu().min(config.batch.mtu);
        config.batch.is_streamed = false;
        Self { link, config }
    }

    pub(crate) fn tx(&self) -> TransportLinkMulticastTx {
        TransportLinkMulticastTx {
            inner: self.clone(),
            buffer: zcondfeat!(
                "transport_compression",
                self.config
                    .batch
                    .is_compression
                    .then_some(BBuf::with_capacity(
                        lz4_flex::block::get_maximum_output_size(self.config.batch.mtu as usize),
                    )),
                None
            ),
        }
    }

    pub(crate) fn rx(&self) -> TransportLinkMulticastRx {
        TransportLinkMulticastRx {
            inner: self.clone(),
        }
    }

    /// Sends the current value.
    ///
    /// # Arguments
    /// * `msg` - Message to schedule or process.
    ///
    /// # Errors
    /// Returns an error if the output cannot be written or sent.

    pub(crate) async fn send(&self, msg: &TransportMessage) -> ZResult<usize> {
        let mut link = self.tx();
        link.send(msg).await
    }

    // pub(crate) async fn recv(&self) -> ZResult<(TransportMessage, Locator)> {
    //     let mut link = self.rx();
    //     link.recv().await
    // }
    ///

    pub(crate) async fn close(&self, reason: Option<u8>) -> ZResult<()> {
        if let Some(reason) = reason {
            // Build the close message
            let message: TransportMessage = Close {
                reason,
                session: false,
            }
            .into();
            // Send the close message on the link
            let _ = self.send(&message).await;
        }
        self.link.close().await
    }
}

impl fmt::Display for TransportLinkMulticast {
    /// Formats the value for display.
    /// # Arguments
    /// * `f` - Mutable formatter whose state is updated by this operation.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.link)
    }
}

impl fmt::Debug for TransportLinkMulticast {
    /// Formats the value for display.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TransportLinkMulticast")
            .field("link", &self.link)
            .field("config", &self.config)
            .finish()
    }
}

/// State required by `TransportLinkMulticastTx` during transport operations.
pub(crate) struct TransportLinkMulticastTx {
    /// Shared internal state.
    pub(crate) inner: TransportLinkMulticast,
    /// Optional buffer used by transport link multicast tx; `None` means the value is not
    /// configured or unavailable.
    pub(crate) buffer: Option<BBuf>,
}

impl TransportLinkMulticastTx {
    /// # Arguments
    /// * `batch` - Mutable wbatch whose state is updated by this operation.

    /// # Errors
    /// An error is returned if finalize, as_slice fails.
    pub(crate) async fn send_batch(&mut self, batch: &mut WBatch) -> ZResult<()> {
        /// Shared `err` used by this module.
        const ERR: &str = "Write error on link: ";

        let res = batch
            .finalize(self.buffer.as_mut())
            .map_err(|_| zerror!("{ERR}{self}"))?;

        let bytes = match res {
            Finalize::Batch => batch.as_slice(),
            Finalize::Buffer => self
                .buffer
                .as_ref()
                .ok_or_else(|| zerror!("Invalid buffer finalization"))?
                .as_slice(),
        };

        // Send the message on the link
        self.inner.link.write_all(bytes).await?;

        Ok(())
    }

    /// # Arguments
    /// * `msg` - Message to schedule or process.
    /// # Errors
    /// An error is returned if encode fails.
    pub(crate) async fn send(&mut self, msg: &TransportMessage) -> ZResult<usize> {
        /// Shared `err` used by this module.
        const ERR: &str = "Write error on link: ";

        // Create the batch for serializing the message
        let mut batch = WBatch::new(self.inner.config.batch);
        batch.encode(msg).map_err(|_| zerror!("{ERR}{self}"))?;
        let len = batch.len() as usize;
        self.send_batch(&mut batch).await?;
        Ok(len)
    }
}

impl fmt::Display for TransportLinkMulticastTx {
    /// Formats the value for display.
    /// # Arguments
    /// * `f` - Mutable formatter whose state is updated by this operation.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl fmt::Debug for TransportLinkMulticastTx {
    /// Formats the value for display.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("TransportLinkMulticastRx");
        s.field("link", &self.inner.link)
            .field("config", &self.inner.config);
        #[cfg(feature = "transport_compression")]
        {
            s.field("buffer", &self.buffer.as_ref().map(|b| b.capacity()));
        }
        s.finish()
    }
}
/// Receive-side wrapper for a multicast transport link.

pub(crate) struct TransportLinkMulticastRx {
    /// Shared internal state.
    pub(crate) inner: TransportLinkMulticast,
}

impl TransportLinkMulticastRx {
    pub async fn recv_batch<C, T>(&self, buff: C) -> ZResult<(RBatch<ZSlice>, Locator)>
    where
        C: Fn() -> T + Copy,
        T: AsMut<[u8]> + ZSliceBuffer + 'static,
    {
        /// Defines the err constant.
        const ERR: &str = "Read error from link: ";

        let mut into = (buff)();
        let (n, locator) = self.inner.link.read(into.as_mut()).await?;
        let buffer = ZSlice::new(Arc::new(into), 0, n).map_err(|_| zerror!("Error"))?;
        let mut batch = RBatch::new(self.inner.config.batch, buffer);
        batch.initialize(buff).map_err(|_| zerror!("{ERR}{self}"))?;
        Ok((batch, locator.into_owned()))
    }

    // pub async fn recv(&mut self) -> ZResult<(TransportMessage, Locator)> {
    //     let mtu = self.inner.config.mtu as usize;
    //     let (mut batch, locator) = self
    //         .recv_batch(|| zenoh_buffers::vec::uninit(mtu).into_boxed_slice())
    //         .await?;
    //     let msg = batch
    //         .decode()
    //         .map_err(|_| zerror!("Decode error on link: {}", self))?;
    //     Ok((msg, locator))
    // }
}

impl fmt::Display for TransportLinkMulticastRx {
    /// Formats the value for display.
    /// # Arguments
    /// * `f` - Mutable formatter whose state is updated by this operation.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl fmt::Debug for TransportLinkMulticastRx {
    /// Formats the value for display.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TransportLinkMulticastRx")
            .field("link", &self.inner.link)
            .field("config", &self.inner.config)
            .finish()
    }
}
/// ***********************************

// TRANSPORT MULTICAST LINK UNIVERSAL
/// ***********************************
pub(super) struct TransportLinkMulticastConfigUniversal {
    /// Protocol version.
    /// Protocol version.
    pub(super) version: u8,
    /// Zenoh identifier of this peer.
    pub(super) zid: ZenohIdProto,
    /// Role advertised by this peer.
    pub(super) whatami: WhatAmI,
    /// Lease duration advertised by the peer.
    pub(super) lease: Duration,
    /// Interval between background task joins.
    pub(super) join_interval: Duration,
    /// Number of bits used for sequence numbers.
    pub(super) sn_resolution: Bits,
    /// Maximum number of messages sent in a batch.
    pub(super) batch_size: BatchSize,
}

// TODO(yuyuan): Introduce TaskTracker or JoinSet and retire handle_tx, handle_rx, and signal_rx.
/// Multicast link wrapper using the universal message pipeline.
#[derive(Clone)]
pub(super) struct TransportLinkMulticastUniversal {
    /// Transport link associated with this state.
    // The underlying link
    /// Transport link associated with this state.
    pub(super) link: TransportLinkMulticast,
    // The transmission pipeline
    /// Optional pipeline used by transport link multicast universal; `None` means the value is not
    /// configured or unavailable.
    pub(super) pipeline: Option<TransmissionPipelineProducer>,
    // The transport this link is associated to
    /// Transport value.
    transport: TransportMulticastInner,
    // The signals to stop TX/RX tasks
    /// Optional handle tx used by transport link multicast universal; `None` means the value is
    /// not configured or unavailable.
    handle_tx: Option<Arc<JoinHandle<()>>>,
    /// Signal rx value.
    signal_rx: Signal,
    /// Optional handle rx used by transport link multicast universal; `None` means the value is
    /// not configured or unavailable.
    handle_rx: Option<Arc<JoinHandle<()>>>,
}

impl TransportLinkMulticastUniversal {
    /// Constructs a usable initial transport link multicast universal value with the defaults
    /// required by the link component.
    ///
    /// # Arguments
    /// * `transport` - Transport used for the operation.
    /// * `link` - Transport link to add, remove, or use.
    pub(super) fn new(
        transport: TransportMulticastInner,
        link: TransportLinkMulticast,
    ) -> TransportLinkMulticastUniversal {
        TransportLinkMulticastUniversal {
            transport,
            link,
            pipeline: None,
            handle_tx: None,
            signal_rx: Signal::new(),
            handle_rx: None,
        }
    }
}

impl TransportLinkMulticastUniversal {
    /// Starts tx.
    ///
    /// # Arguments
    /// * `config` - Configuration applied to this operation.
    pub(super) fn start_tx(
        &mut self,
        config: TransportLinkMulticastConfigUniversal,
        priority_tx: Arc<[TransportPriorityTx]>,
    ) {
        let initial_sns: Vec<PrioritySn> = priority_tx
            .iter()
            .map(|x| PrioritySn {
                reliable: {
                    let sn = zlock!(x.reliable).sn.now();
                    if sn == 0 {
                        config.sn_resolution.mask() as TransportSn
                    } else {
                        sn - 1
                    }
                },
                best_effort: {
                    let sn = zlock!(x.best_effort).sn.now();
                    if sn == 0 {
                        config.sn_resolution.mask() as TransportSn
                    } else {
                        sn - 1
                    }
                },
            })
            .collect();

        if self.handle_tx.is_none() {
            let tpc = TransmissionPipelineConf {
                batch: self.link.config.batch,
                queue_size: self.transport.manager.config.queue_size,
                wait_before_drop: self.transport.manager.config.wait_before_drop,
                max_wait_before_drop_fragments: self
                    .transport
                    .manager
                    .config
                    .max_wait_before_drop_fragments,
                wait_before_close: self.transport.manager.config.wait_before_close,
                batching_enabled: self.transport.manager.config.batching,
                batching_time_limit: self.transport.manager.config.queue_backoff,
                queue_alloc: self.transport.manager.config.queue_alloc,
            };
            // The pipeline
            let (producer, consumer) = TransmissionPipeline::make(tpc, &priority_tx, false);
            self.pipeline = Some(producer);

            // Spawn the TX task
            let c_link = self.link.clone();
            let c_transport = self.transport.clone();

            let handle = zenoh_runtime::ZRuntime::TX.spawn(async move {
                let res = tx_task(
                    consumer,
                    c_link.tx(),
                    config,
                    initial_sns,
                    #[cfg(feature = "stats")]
                    c_transport.link_stats.clone(),
                )
                .await;
                if let Err(e) = res {
                    tracing::debug!("TX task failed: {}", e);
                    // Spawn a task to avoid a deadlock waiting for this same task
                    // to finish in the close() joining its handle
                    zenoh_runtime::ZRuntime::Net.spawn(async move { c_transport.delete().await });
                }
            });
            self.handle_tx = Some(Arc::new(handle));
        }
    }

    /// Stops tx.

    pub(super) fn stop_tx(&mut self) {
        if let Some(pipeline) = self.pipeline.as_ref() {
            pipeline.disable();
        }
    }

    /// Starts rx.

    pub(super) fn start_rx(&mut self, batch_size: BatchSize) {
        if self.handle_rx.is_none() {
            // Spawn the RX task
            let c_link = self.link.clone();
            let c_transport = self.transport.clone();
            let c_signal = self.signal_rx.clone();
            let c_rx_buffer_size = self.transport.manager.config.link_rx_buffer_size;

            let handle = zenoh_runtime::ZRuntime::RX.spawn(async move {
                // Start the consume task
                let res = rx_task(
                    c_link.rx(),
                    c_transport.clone(),
                    c_signal.clone(),
                    c_rx_buffer_size,
                    batch_size,
                )
                .await;
                c_signal.trigger();
                if let Err(e) = res {
                    tracing::debug!("RX task failed: {}", e);
                    // Spawn a task to avoid a deadlock waiting for this same task
                    // to finish in the close() joining its handle
                    zenoh_runtime::ZRuntime::Net.spawn(async move { c_transport.delete().await });
                }
            });
            self.handle_rx = Some(Arc::new(handle));
        }
    }

    /// Stops rx.

    pub(super) fn stop_rx(&mut self) {
        self.signal_rx.trigger();
    }

    ///

    /// # Errors
    /// An error is returned if a fallible operation fails.
    ///
    /// # Panics
    /// Panics if an internal `Result` or `Option` has no usable value.
    pub(super) async fn close(mut self) -> ZResult<()> {
        tracing::trace!("{}: closing", self.link);
        self.stop_rx();
        if let Some(handle) = self.handle_rx.take() {
            // It is safe to unwrap the Arc since we have the ownership of the whole link
            let handle_rx = Arc::try_unwrap(handle).unwrap();
            handle_rx.await?;
        }

        self.stop_tx();
        if let Some(handle) = self.handle_tx.take() {
            // It is safe to unwrap the Arc since we have the ownership of the whole link
            let handle_tx = Arc::try_unwrap(handle).unwrap();
            handle_tx.await?;
        }

        self.link.close(None).await
    }
}

/// **********************************
// TASKS
/// **********************************
async fn tx_task(
    mut pipeline: TransmissionPipelineConsumer,
    mut link: TransportLinkMulticastTx,
    config: TransportLinkMulticastConfigUniversal,
    mut last_sns: Vec<PrioritySn>,
    #[cfg(feature = "stats")] stats: zenoh_stats::LinkStats,
) -> ZResult<()> {
    async fn join(last_join: Instant, join_interval: Duration) {
        let now = Instant::now();
        let target = last_join + join_interval;
        if now < target {
            let left = target - now;
            tokio::time::sleep(left).await;
        }
    }

    let mut last_join = Instant::now().checked_sub(config.join_interval).unwrap();
    loop {
        tokio::select! {
            res = pipeline.pull() => {
                match res {
                    Some((mut batch, priority)) => {
                        // Send the buffer on the link
                        link.send_batch(&mut batch).await?;
                        // Keep track of next SNs
                        if let Some(sn) = batch.codec.latest_sn.reliable {
                            last_sns[priority as usize].reliable = sn;
                        }
                        if let Some(sn) = batch.codec.latest_sn.best_effort {
                            last_sns[priority as usize].best_effort = sn;
                        }
                        #[cfg(feature = "stats")]
                        {
                            stats.inc_bytes(zenoh_stats::Tx,  batch.len() as u64);
                            stats.inc_transport_message(zenoh_stats::Tx, batch.stats.t_msgs as u64);
                        }
                        // Reinsert the batch into the queue
                        pipeline.refill(batch, priority);
                    }
                    None => {
                        // Drain the transmission pipeline and write remaining bytes on the wire
                        let mut batches = pipeline.drain();
                        for (mut b, _) in batches.drain(..) {
                            tokio::time::timeout(config.join_interval, link.send_batch(&mut b))
                                .await
                                .map_err(|_| {
                                    zerror!(
                                        "{}: flush failed after {} ms",
                                        link,
                                        config.join_interval.as_millis()
                                    )
                                })??;

                            #[cfg(feature = "stats")]
                            {
                                stats.inc_bytes(zenoh_stats::Tx,  b.len() as u64);
                                stats.inc_transport_message(zenoh_stats::Tx,  b.stats.t_msgs as u64);
                            }
                        }
                        break;
                    }

                }
            }

            _ = join(last_join, config.join_interval) => {
                let next_sns = last_sns
                    .iter()
                    .map(|c| PrioritySn {
                        reliable: (1 + c.reliable) & config.sn_resolution.mask() as TransportSn,
                        best_effort: (1 + c.best_effort)
                            & config.sn_resolution.mask() as TransportSn,
                    })
                    .collect::<Vec<PrioritySn>>();
                let (next_sn, ext_qos) = if next_sns.len() == Priority::NUM {
                    let tmp: [PrioritySn; Priority::NUM] = next_sns.try_into().unwrap();
                    (PrioritySn::DEFAULT, Some(Box::new(tmp)))
                } else {
                    (next_sns[0], None)
                };
                let message: TransportMessage = Join {
                    version: config.version,
                    whatami: config.whatami,
                    zid: config.zid,
                    resolution: Resolution::default(),
                    batch_size: config.batch_size,
                    lease: config.lease,
                    next_sn,
                    ext_qos,
                    ext_shm: None,
                    ext_patch: PatchType::CURRENT
                }
                .into();

                #[allow(unused_variables)] // Used when stats feature is enabled
                let n = link.send(&message).await?;
                #[cfg(feature = "stats")]
                {
                    stats.inc_bytes(zenoh_stats::Tx, n as u64);
                    stats.inc_transport_message(zenoh_stats::Tx,  1);
                }

                last_join = Instant::now();

            }
        }
    }

    Ok(())
}

/// # Errors
/// An error is returned if read_messages fails.
async fn rx_task(
    mut link: TransportLinkMulticastRx,
    transport: TransportMulticastInner,
    signal: Signal,
    rx_buffer_size: usize,
    batch_size: BatchSize,
) -> ZResult<()> {
    /// # Errors
    /// An error is returned if a fallible operation fails.
    async fn read<T, F>(
        link: &mut TransportLinkMulticastRx,
        pool: &RecyclingObjectPool<T, F>,
    ) -> ZResult<(RBatch<ZSlice>, Locator)>
    where
        T: ZSliceBuffer + 'static,
        F: Fn() -> T,
        RecyclingObject<T>: AsMut<[u8]> + ZSliceBuffer,
    {
        let (rbatch, locator) = link
            .recv_batch(|| pool.try_take().unwrap_or_else(|| pool.alloc()))
            .await?;
        Ok((rbatch, locator))
    }

    // The pool of buffers
    let mtu = link.inner.config.batch.mtu as usize;
    let mut n = rx_buffer_size / mtu;
    if n == 0 {
        tracing::debug!(
            "RX configured buffer of {rx_buffer_size} bytes is too small for {link} that has an MTU of {mtu} bytes. Defaulting to {mtu} bytes for RX buffer."
        );
        n = 1;
    }

    let pool = RecyclingObjectPool::new(n, || vec![0_u8; mtu].into_boxed_slice());
    loop {
        tokio::select! {
            _ = signal.wait() => break,
            res = read(&mut link, &pool) => {
                let (batch, locator) = res?;

                #[cfg(feature = "stats")]
                transport.link_stats.inc_bytes(zenoh_stats::Rx, batch.len() as u64);

                // Deserialize all the messages from the current ZBuf
                transport.read_messages(
                    batch,
                    locator,
                    batch_size,
                    #[cfg(feature = "stats")]
                    &transport.link_stats,
                )?;
            }
        }
    }
    Ok(())
}
