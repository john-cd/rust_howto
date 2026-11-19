//! Organizes the establishment subsystem and declares its child modules.
//! Parent components use this module to access the subsystem functionality.
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

/// Accepts and validates an incoming unicast transport handshake.
pub(crate) mod accept;

/// Creates and validates cookies used to protect incoming transport handshakes.
pub(super) mod cookie;

/// Organizes the ext subsystem and declares its child modules.
pub mod ext;

/// Opens an outgoing unicast transport and completes its establishment handshake.
pub(crate) mod open;

use async_trait::async_trait;
use cookie::*;
use sha3::Shake128;
use sha3::digest::ExtendableOutput;
use sha3::digest::Update;
use sha3::digest::XofReader;
use zenoh_protocol::core::Field;
use zenoh_protocol::core::Resolution;
use zenoh_protocol::core::ZenohIdProto;
use zenoh_protocol::transport::TransportSn;

use crate::common::seq_num;

/// **********************************
// TRAITS

/// **********************************
#[async_trait]
pub trait OpenFsm {
    /// Type of the error values produced or consumed by this trait.
    type Error;

    /// Type of the send init syn in values produced or consumed by this trait.
    type SendInitSynIn;
    /// Type of the send init syn out values produced or consumed by this trait.
    type SendInitSynOut;

    /// Builds the opening peer's initial SYN message and extension values.
    ///
    /// # Arguments
    ///
    /// * `input` - Opening state and local settings needed to construct InitSyn.
    async fn send_init_syn(
        self,
        input: Self::SendInitSynIn,
    ) -> Result<Self::SendInitSynOut, Self::Error>;

    /// Type of the recv init ack in values produced or consumed by this trait.
    type RecvInitAckIn;
    /// Type of the recv init ack out values produced or consumed by this trait.
    type RecvInitAckOut;

    /// Validates the peer's InitAck and updates the opening handshake state.
    ///
    /// # Arguments
    ///
    /// * `input` - Link and opening state to update from the received InitAck.
    async fn recv_init_ack(
        self,
        input: Self::RecvInitAckIn,
    ) -> Result<Self::RecvInitAckOut, Self::Error>;

    /// Type of the send open syn in values produced or consumed by this trait.
    type SendOpenSynIn;
    /// Type of the send open syn out values produced or consumed by this trait.
    type SendOpenSynOut;

    /// Builds the opening peer's OpenSyn message and extension values.
    ///
    /// # Arguments
    ///
    /// * `input` - Link, negotiated opening state, and local OpenSyn settings.
    async fn send_open_syn(
        self,
        input: Self::SendOpenSynIn,
    ) -> Result<Self::SendOpenSynOut, Self::Error>;

    /// Type of the recv open ack in values produced or consumed by this trait.
    type RecvOpenAckIn;
    /// Type of the recv open ack out values produced or consumed by this trait.
    type RecvOpenAckOut;

    /// Validates the peer's OpenAck and completes opening-side negotiation.
    ///
    /// # Arguments
    ///
    /// * `input` - Link and opening state to update from the received OpenAck.
    async fn recv_open_ack(
        self,
        input: Self::RecvOpenAckIn,
    ) -> Result<Self::RecvOpenAckOut, Self::Error>;
}

/// Specifies the operations implementations provide to establishment callers.
#[async_trait]
pub trait AcceptFsm {
    /// Type of the error values produced or consumed by this trait.
    type Error;

    /// Type of the recv init syn in values produced or consumed by this trait.
    type RecvInitSynIn;
    /// Type of the recv init syn out values produced or consumed by this trait.
    type RecvInitSynOut;

    /// Validates the opening peer's InitSyn and records its negotiated settings.
    ///
    /// # Arguments
    ///
    /// * `input` - Accepting state and received InitSyn fields to process.
    async fn recv_init_syn(
        self,
        input: Self::RecvInitSynIn,
    ) -> Result<Self::RecvInitSynOut, Self::Error>;

    /// Type of the send init ack in values produced or consumed by this trait.
    type SendInitAckIn;
    /// Type of the send init ack out values produced or consumed by this trait.
    type SendInitAckOut;

    /// Builds the accepting peer's InitAck message and extension values.
    ///
    /// # Arguments
    ///
    /// * `input` - Accepting state and local InitAck settings.
    async fn send_init_ack(
        self,
        input: Self::SendInitAckIn,
    ) -> Result<Self::SendInitAckOut, Self::Error>;

    /// Type of the recv open syn in values produced or consumed by this trait.
    type RecvOpenSynIn;
    /// Type of the recv open syn out values produced or consumed by this trait.
    type RecvOpenSynOut;

    /// Validates the opening peer's OpenSyn and completes accept-side negotiation.
    ///
    /// # Arguments
    ///
    /// * `input` - Accepting state and received OpenSyn fields to process.
    async fn recv_open_syn(
        self,
        input: Self::RecvOpenSynIn,
    ) -> Result<Self::RecvOpenSynOut, Self::Error>;

    /// Type of the send open ack in values produced or consumed by this trait.
    type SendOpenAckIn;
    /// Type of the send open ack out values produced or consumed by this trait.
    type SendOpenAckOut;

    /// Builds the accepting peer's OpenAck message and extension values.
    ///
    /// # Arguments
    ///
    /// * `input` - Accepting state and local OpenAck settings.
    async fn send_open_ack(
        self,
        input: Self::SendOpenAckIn,
    ) -> Result<Self::SendOpenAckOut, Self::Error>;
}

/// **********************************
// FUNCTIONS

/// **********************************
pub(super) fn compute_sn(
    zid1: ZenohIdProto,
    zid2: ZenohIdProto,
    resolution: Resolution,
) -> TransportSn {
    // Create a random yet deterministic initial_sn.
    // In case of multilink it's important that the same initial_sn is used for every connection
    // attempt. Instead of storing the state everywhere, we make sure that the we always compute
    // the same initial_sn.
    let mut hasher = Shake128::default();
    hasher.update(&zid1.to_le_bytes()[..zid1.size()]);
    hasher.update(&zid2.to_le_bytes()[..zid2.size()]);
    let mut array = (0 as TransportSn).to_le_bytes();
    hasher.finalize_xof().read(&mut array);
    TransportSn::from_le_bytes(array) & seq_num::get_mask(resolution.get(Field::FrameSN))
}
