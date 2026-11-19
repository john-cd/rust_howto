//! Tracks sequence numbers and queues for transport priorities.
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
//
use std::sync::Arc;
use std::sync::Mutex;

use zenoh_core::zlock;
use zenoh_protocol::core::Bits;
use zenoh_protocol::core::Reliability;
use zenoh_protocol::transport::PrioritySn;
use zenoh_protocol::transport::TransportSn;
use zenoh_result::ZResult;

use super::defragmentation::DefragBuffer;
use super::seq_num::SeqNum;
use super::seq_num::SeqNumGenerator;

/// Data for transport channel tx operations, including sn.
#[derive(Debug)]
pub(crate) struct TransportChannelTx {
    /// Sequence number used to order transport messages.
    pub(crate) sn: SeqNumGenerator,
}

impl TransportChannelTx {
    /// Constructs the value from the supplied transport configuration.
    pub(crate) fn make(resolution: Bits) -> ZResult<TransportChannelTx> {
        let tch = TransportChannelTx {
            sn: SeqNumGenerator::make(0, resolution)?,
        };
        Ok(tch)
    }

    /// Synchronizes the local sequence state with the supplied transport state.
    pub(crate) fn sync(&mut self, sn: TransportSn) -> ZResult<()> {
        self.sn.set(sn)
    }
}

/// Data for transport channel rx operations, including sn and defrag.
#[derive(Debug)]
pub(crate) struct TransportChannelRx {
    /// Sequence number used to order transport messages.
    pub(crate) sn: SeqNum,
    /// Reassembly buffer that collects fragments until a complete message is available.
    pub(crate) defrag: DefragBuffer,
}

impl TransportChannelRx {
    /// Constructs the value from the supplied transport configuration.
    pub(crate) fn make(
        reliability: Reliability,
        resolution: Bits,
        defrag_buff_size: usize,
    ) -> ZResult<TransportChannelRx> {
        let sn = SeqNum::make(0, resolution)?;
        let defrag = DefragBuffer::make(reliability, resolution, defrag_buff_size)?;
        let tch = TransportChannelRx { sn, defrag };
        Ok(tch)
    }

    /// Synchronizes the local sequence state with the supplied transport state.
    pub(crate) fn sync(&mut self, sn: TransportSn) -> ZResult<()> {
        // Set the sequence number in the state as it had received a message with sn - 1
        let sn = if sn == 0 {
            self.sn.resolution() - 1
        } else {
            sn - 1
        };

        self.sn.set(sn)?;
        self.defrag.sync(sn)
    }
}

/// Data for transport priority tx operations, including reliable and best effort.
#[derive(Clone, Debug)]
pub(crate) struct TransportPriorityTx {
    /// Whether this transport channel uses reliable delivery.
    pub(crate) reliable: Arc<Mutex<TransportChannelTx>>,
    /// Best-effort channel used when reliable delivery is unnecessary.
    pub(crate) best_effort: Arc<Mutex<TransportChannelTx>>,
}

impl TransportPriorityTx {
    /// Constructs the value from the supplied transport configuration.
    pub(crate) fn make(resolution: Bits) -> ZResult<TransportPriorityTx> {
        let rch = TransportChannelTx::make(resolution)?;
        let bch = TransportChannelTx::make(resolution)?;
        let ctx = TransportPriorityTx {
            reliable: Arc::new(Mutex::new(rch)),
            best_effort: Arc::new(Mutex::new(bch)),
        };
        Ok(ctx)
    }

    /// Synchronizes the local sequence state with the supplied transport state.
    pub(crate) fn sync(&self, sn: PrioritySn) -> ZResult<()> {
        zlock!(self.reliable).sync(sn.reliable)?;
        zlock!(self.best_effort).sync(sn.best_effort)
    }
}

/// Data for transport priority rx operations, including reliable and best effort.
#[derive(Clone, Debug)]
pub(crate) struct TransportPriorityRx {
    /// Whether this transport channel uses reliable delivery.
    pub(crate) reliable: Arc<Mutex<TransportChannelRx>>,
    /// Best-effort channel used when reliable delivery is unnecessary.
    pub(crate) best_effort: Arc<Mutex<TransportChannelRx>>,
}

impl TransportPriorityRx {
    /// Constructs the value from the supplied transport configuration.
    pub(crate) fn make(resolution: Bits, defrag_buff_size: usize) -> ZResult<TransportPriorityRx> {
        let rch = TransportChannelRx::make(Reliability::Reliable, resolution, defrag_buff_size)?;
        let bch = TransportChannelRx::make(Reliability::BestEffort, resolution, defrag_buff_size)?;
        let ctr = TransportPriorityRx {
            reliable: Arc::new(Mutex::new(rch)),
            best_effort: Arc::new(Mutex::new(bch)),
        };
        Ok(ctr)
    }

    /// Synchronizes the local sequence state with the supplied transport state.
    pub(crate) fn sync(&self, sn: PrioritySn) -> ZResult<()> {
        zlock!(self.reliable).sync(sn.reliable)?;
        zlock!(self.best_effort).sync(sn.best_effort)
    }
}
