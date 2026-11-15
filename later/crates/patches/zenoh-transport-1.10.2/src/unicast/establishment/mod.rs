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
    type Error;

    type SendInitSynIn;
    type SendInitSynOut;
    /// Sends init syn.
    /// # Arguments
    /// * `input` - Input data whose fields determine the requested open fsm operation.
    async fn send_init_syn(
        self,
        input: Self::SendInitSynIn,
    ) -> Result<Self::SendInitSynOut, Self::Error>;

    type RecvInitAckIn;
    type RecvInitAckOut;
    /// Receives init ack.
    /// # Arguments
    /// * `input` - Input data whose fields determine the requested open fsm operation.
    async fn recv_init_ack(
        self,
        input: Self::RecvInitAckIn,
    ) -> Result<Self::RecvInitAckOut, Self::Error>;

    type SendOpenSynIn;
    type SendOpenSynOut;
    /// Sends open syn.
    /// # Arguments
    /// * `input` - Input data whose fields determine the requested open fsm operation.
    async fn send_open_syn(
        self,
        input: Self::SendOpenSynIn,
    ) -> Result<Self::SendOpenSynOut, Self::Error>;

    type RecvOpenAckIn;
    type RecvOpenAckOut;
    /// Receives open ack.
    /// # Arguments
    /// * `input` - Input data whose fields determine the requested open fsm operation.
    async fn recv_open_ack(
        self,
        input: Self::RecvOpenAckIn,
    ) -> Result<Self::RecvOpenAckOut, Self::Error>;
}

/// Specifies the operations implementations provide to establishment callers.
#[async_trait]
pub trait AcceptFsm {
    type Error;

    type RecvInitSynIn;
    type RecvInitSynOut;
    /// Receives init syn.
    /// # Arguments
    /// * `input` - Input data whose fields determine the requested accept fsm operation.
    async fn recv_init_syn(
        self,
        input: Self::RecvInitSynIn,
    ) -> Result<Self::RecvInitSynOut, Self::Error>;

    type SendInitAckIn;
    type SendInitAckOut;
    /// Sends init ack.
    /// # Arguments
    /// * `input` - Input data whose fields determine the requested accept fsm operation.
    async fn send_init_ack(
        self,
        input: Self::SendInitAckIn,
    ) -> Result<Self::SendInitAckOut, Self::Error>;

    type RecvOpenSynIn;
    type RecvOpenSynOut;
    /// Receives open syn.
    /// # Arguments
    /// * `input` - Input data whose fields determine the requested accept fsm operation.
    async fn recv_open_syn(
        self,
        input: Self::RecvOpenSynIn,
    ) -> Result<Self::RecvOpenSynOut, Self::Error>;

    type SendOpenAckIn;
    type SendOpenAckOut;
    /// Sends open ack.
    /// # Arguments
    /// * `input` - Input data whose fields determine the requested accept fsm operation.
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
