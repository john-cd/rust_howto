//! Builds and validates shared-memory segment metadata for a unicast transport.
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
use std::ops::Deref;
use std::sync::Arc;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::SeqCst;

use zenoh_buffers::reader::DidntRead;
use zenoh_buffers::reader::Reader;
use zenoh_buffers::writer::DidntWrite;
use zenoh_buffers::writer::Writer;
use zenoh_codec::RCodec;
use zenoh_codec::WCodec;
use zenoh_codec::Zenoh080;
use zenoh_core::bail;
use zenoh_core::zerror;
use zenoh_core::zlock;
use zenoh_result::ZResult;
use zenoh_shm::api::common::types::ProtocolID;
use zenoh_shm::posix_shm::struct_in_shm::StructInSHM;
use zenoh_shm::version::SHM_VERSION;

/// **********************************
// Segment

/// **********************************
pub(crate) type AuthSegmentID = u32;
/// Alias for `u64` used to represent auth challenge.
pub(crate) type AuthChallenge = u64;
/// Alias for `u16` used to represent shm counter id.
pub(crate) type ShmCounterID = u16;

/// Data for shm transport metadata operations, including id count and challenge.
#[derive(Debug)]
#[repr(C)]
struct ShmTransportMetadata {
    /// Number of identifiers allocated in this shared-memory segment.
    id_count: u64,
    /// Challenge value exchanged to authenticate the peer.
    challenge: u64,
    /// Shared-memory metadata format version used to validate peer compatibility.
    version: u64,
    /// Protocol identifiers advertised by peers that may use this shared-memory segment.
    protocols: [ProtocolID; 256],
    // Allows to service 351 priority-enabled peers
    // TODO: Support growing counters array based on segment memory overcommit to control memory
    // usage
    shm_counters: [AtomicU32; 762 + 2048],
}

impl ShmTransportMetadata {
    /// Handles the validate behavior for this component.
    fn validate(&self, expected_challenge: AuthChallenge, s: &str) -> bool {
        if self.challenge != expected_challenge {
            tracing::debug!(
                "{} Challenge mismatch: expected: {}, found in shm: {}.",
                s,
                expected_challenge,
                self.challenge
            );
            return false;
        }

        if self.version != SHM_VERSION {
            tracing::warn!(
                "{} Version mismatch: ours: {}, theirs: {}.",
                s,
                SHM_VERSION,
                self.version
            );
            return false;
        }

        if self.id_count as usize > self.protocols.len() {
            tracing::warn!(
                "{} Protocol count mismatch: ours: {}, theirs: {}.",
                s,
                self.protocols.len(),
                self.id_count
            );
            return false;
        }

        true
    }

    /// Handles the protocols behavior for this component.
    fn protocols(&self) -> &[ProtocolID] {
        &self.protocols[..self.id_count as usize]
    }

    /// Handles the counter behavior for this component.
    fn counter(&self, id: ShmCounterID) -> &AtomicU32 {
        &self.shm_counters[id as usize]
    }

    /// Validates the supplied counter index before it is used.
    pub fn validate_counter_index(&self, id: ShmCounterID) -> bool {
        (id as usize) < self.shm_counters.len()
    }
}

/// Data for shm transport metadata segment operations, including data.
#[derive(Debug, PartialEq, Eq)]
struct ShmTransportMetadataSegment {
    /// Payload bytes shared with the receiving peer.
    data: StructInSHM<AuthSegmentID, ShmTransportMetadata>,
}

impl ShmTransportMetadataSegment {
    /// Creates the value from the supplied configuration and inputs.
    fn create(challenge: AuthChallenge, shm_protocols: &[ProtocolID]) -> ZResult<Self> {
        let mut data: StructInSHM<AuthSegmentID, ShmTransportMetadata> = StructInSHM::create()?;

        if data.protocols.len() < shm_protocols.len() {
            bail!(
                "Too many protocols: {}. Max is {}",
                shm_protocols.len(),
                data.protocols.len()
            );
        }

        data.id_count = shm_protocols.len() as u64;
        data.challenge = challenge;
        data.version = SHM_VERSION;
        data.protocols[..shm_protocols.len()].copy_from_slice(shm_protocols);

        // We intentionally do not reset the counters here, as they are expected to be reset when
        // leased. This allows to rely on memory overcommit mechanics and save some physical
        // memory pages for most setups that don't really utilize all the counters
        // data.shm_counters
        //     .iter()
        //     .for_each(|counter| counter.store(0, std::sync::atomic::Ordering::Relaxed));

        Ok(Self { data })
    }

    /// Opens the transport using the negotiated session parameters.
    fn open(id: AuthSegmentID) -> ZResult<Self> {
        let data = StructInSHM::open(id)?;
        Ok(Self { data })
    }
}

/// Data for tx auth segment operations, including segment and available shm counters.
pub struct TXAuthSegment {
    /// segment value tracked by this type.
    segment: ShmTransportMetadataSegment,
    /// Shared-memory counters available for peer priorities.
    available_shm_counters: Arc<std::sync::Mutex<VecDeque<ShmCounterID>>>,
}

impl std::fmt::Debug for TXAuthSegment {
    /// Formats this value for human-readable output.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TXAuthSegment")
            .field("segment", &self.segment)
            .field(
                "available_shm_counters",
                &zlock!(self.available_shm_counters).len(),
            )
            .finish()
    }
}

impl TXAuthSegment {
    /// Creates the value from the supplied configuration and inputs.
    pub fn create(challenge: AuthChallenge, shm_protocols: &[ProtocolID]) -> ZResult<Self> {
        let segment = ShmTransportMetadataSegment::create(challenge, shm_protocols)?;
        let available_shm_counters = Arc::new(std::sync::Mutex::new(
            (0..segment.data.shm_counters.len())
                .map(|index| index as ShmCounterID)
                .collect(),
        ));

        Ok(Self {
            segment,
            available_shm_counters,
        })
    }

    /// Handles the validate behavior for this component.
    pub fn validate(&self, expected_challenge: AuthChallenge, s: &str) -> bool {
        self.segment.data.validate(expected_challenge, s)
    }

    /// Handles the segment id behavior for this component.
    pub fn segment_id(&self) -> AuthSegmentID {
        self.segment.data.id()
    }
}

/// Data for shm tx counter lease operations, including segment and counter index.
#[derive(Debug)]
pub struct ShmTXCounterLease {
    /// segment value tracked by this type.
    segment: Arc<TXAuthSegment>,
    /// Index of the per-priority shared-memory counter.
    counter_index: ShmCounterID,
}

impl ShmTXCounterLease {
    /// Creates a new shm tx counter lease with the supplied configuration.
    pub fn new(segment: Arc<TXAuthSegment>) -> ZResult<Self> {
        let index = zlock!(segment.available_shm_counters).pop_front();
        index
            .map(|counter_index| {
                // Reset counter to 0 when lease is created
                segment.segment.data.counter(counter_index).store(0, SeqCst);
                Self {
                    segment,
                    counter_index,
                }
            })
            .ok_or_else(|| zerror!("No available SHM counters").into())
    }

    /// Handles the id behavior for this component.
    pub fn id(&self) -> ShmCounterID {
        self.counter_index
    }

    /// Adds the supplied value to the current collection or state.
    pub fn add(&self, val: u32) {
        self.segment
            .segment
            .data
            .counter(self.counter_index)
            .fetch_add(val, SeqCst);
    }

    /// Handles the sub behavior for this component.
    pub fn sub(&self, val: u32) {
        self.segment
            .segment
            .data
            .counter(self.counter_index)
            .fetch_sub(val, SeqCst);
    }

    /// Handles the counter behavior for this component.
    pub fn counter(&self) -> u32 {
        self.segment
            .segment
            .data
            .counter(self.counter_index)
            .load(std::sync::atomic::Ordering::Relaxed)
    }
}

impl Drop for ShmTXCounterLease {
    /// Releases the resources owned by this value.
    fn drop(&mut self) {
        zlock!(self.segment.available_shm_counters).push_front(self.counter_index);
    }
}

/// Data for rx auth segment operations, including segment.
#[derive(Debug, PartialEq, Eq)]
pub struct RXAuthSegment {
    /// segment value tracked by this type.
    segment: ShmTransportMetadataSegment,
}

impl RXAuthSegment {
    /// Opens the transport using the negotiated session parameters.
    pub fn open(id: AuthSegmentID) -> ZResult<Self> {
        let segment = ShmTransportMetadataSegment::open(id)?;
        Ok(Self { segment })
    }

    /// Handles the challenge behavior for this component.
    pub fn challenge(&self) -> AuthChallenge {
        self.segment.data.challenge
    }

    /// Handles the protocols behavior for this component.
    pub fn protocols(&self) -> &[ProtocolID] {
        self.segment.data.protocols()
    }
}

// Codec
impl<W> WCodec<&RXAuthSegment, &mut W> for Zenoh080
where
    W: Writer,
{
    /// Alias for `Result<(), DidntWrite>` used to represent output.
    type Output = Result<(), DidntWrite>;

    /// Writes the supplied value to the configured output sink.
    fn write(self, writer: &mut W, x: &RXAuthSegment) -> Self::Output {
        self.write(writer, &x.segment.data)
    }
}

impl<R> RCodec<RXAuthSegment, &mut R> for Zenoh080
where
    R: Reader,
{
    /// Alias for `DidntRead` used to represent error.
    type Error = DidntRead;

    /// Reads the next value from the supplied input source.
    fn read(self, reader: &mut R) -> Result<RXAuthSegment, Self::Error> {
        let segment_data = self.read(&mut *reader)?;
        Ok(RXAuthSegment {
            segment: ShmTransportMetadataSegment { data: segment_data },
        })
    }
}

/// Data for shm rx counter lease operations, including segment and counter index.
#[derive(Debug, PartialEq, Eq)]
pub struct ShmRXCounterLease {
    /// segment value tracked by this type.
    segment: Arc<RXAuthSegment>,
    /// Index of the per-priority shared-memory counter.
    counter_index: ShmCounterID,
}

impl ShmRXCounterLease {
    /// Creates a new shm rx counter lease with the supplied configuration.
    pub fn new(segment: Arc<RXAuthSegment>, counter_index: ShmCounterID) -> ZResult<Self> {
        segment
            .segment
            .data
            .validate_counter_index(counter_index)
            .then_some(Self {
                segment,
                counter_index,
            })
            .ok_or_else(|| zerror!("Invalid counter index: {}", counter_index).into())
    }

    /// Handles the counter decrease behavior for this component.
    pub fn counter_decrease(&self) {
        self.segment
            .segment
            .data
            .counter(self.counter_index)
            .fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
    }

    /// Handles the segment behavior for this component.
    pub fn segment(&self) -> &RXAuthSegment {
        &self.segment
    }
}

impl<W> WCodec<&ShmRXCounterLease, &mut W> for Zenoh080
where
    W: Writer,
{
    /// Alias for `Result<(), DidntWrite>` used to represent output.
    type Output = Result<(), DidntWrite>;

    /// Writes the supplied value to the configured output sink.
    fn write(self, writer: &mut W, x: &ShmRXCounterLease) -> Self::Output {
        self.write(&mut *writer, x.segment.deref())?;
        self.write(writer, &x.counter_index)
    }
}

impl<R> RCodec<ShmRXCounterLease, &mut R> for Zenoh080
where
    R: Reader,
{
    /// Alias for `DidntRead` used to represent error.
    type Error = DidntRead;

    /// Reads the next value from the supplied input source.
    fn read(self, reader: &mut R) -> Result<ShmRXCounterLease, Self::Error> {
        let segment_data = self.read(&mut *reader)?;
        let counter_index = self.read(reader)?;
        Ok(ShmRXCounterLease {
            segment: Arc::new(RXAuthSegment {
                segment: ShmTransportMetadataSegment { data: segment_data },
            }),
            counter_index,
        })
    }
}
