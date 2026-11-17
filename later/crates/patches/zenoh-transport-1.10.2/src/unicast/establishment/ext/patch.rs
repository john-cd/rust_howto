//! Negotiates protocol patch settings during unicast establishment.
//!
//! TODO Reviewed ??-??-2026.

// Copyright (c) 2022 ZettaScale Technology
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
use std::cmp::min;
use std::marker::PhantomData;

use async_trait::async_trait;
use zenoh_buffers::reader::DidntRead;
use zenoh_buffers::reader::Reader;
use zenoh_buffers::writer::DidntWrite;
use zenoh_buffers::writer::Writer;
use zenoh_codec::RCodec;
use zenoh_codec::WCodec;
use zenoh_codec::Zenoh080;
use zenoh_protocol::transport::init::ext::PatchType;
use zenoh_result::Error as ZError;
use zenoh_result::bail;

use crate::unicast::establishment::AcceptFsm;
use crate::unicast::establishment::OpenFsm;

// Extension Fsm
pub(crate) struct PatchFsm<'a> {
    /// a value tracked by this type.
    _a: PhantomData<&'a ()>,
}

impl PatchFsm<'_> {
    /// Creates a new patch fsm with the supplied configuration.
    pub(crate) const fn new() -> Self {
        Self { _a: PhantomData }
    }
}

/// **********************************
// OPEN

/// **********************************
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct StateOpen {
    /// Patch type selected during the transport open handshake.
    patch: PatchType,
}

impl StateOpen {
    /// Creates a new state open with the supplied configuration.
    pub(crate) const fn new() -> Self {
        Self {
            patch: PatchType::NONE,
        }
    }

    /// Returns the value requested from this object.
    pub(crate) const fn get(&self) -> PatchType {
        self.patch
    }
}

#[async_trait]
impl<'a> OpenFsm for &'a PatchFsm<'a> {
    /// Alias for `ZError` used to represent error.
    type Error = ZError;
    /// Alias for `(&'a mut StateOpen, PatchType)` used to represent recv init ack in.
    type RecvInitAckIn = (&'a mut StateOpen, PatchType);
    /// Alias for `()` used to represent recv init ack out.
    type RecvInitAckOut = ();
    /// Alias for `(&'a mut StateOpen, ())` used to represent recv open ack in.
    type RecvOpenAckIn = (&'a mut StateOpen, ());
    /// Alias for `()` used to represent recv open ack out.
    type RecvOpenAckOut = ();
    /// Alias for `&'a StateOpen` used to represent send init syn in.
    type SendInitSynIn = &'a StateOpen;
    /// Alias for `PatchType` used to represent send init syn out.
    type SendInitSynOut = PatchType;
    /// Alias for `&'a StateOpen` used to represent send open syn in.
    type SendOpenSynIn = &'a StateOpen;
    /// Alias for `()` used to represent send open syn out.
    type SendOpenSynOut = ();

    /// Sends the init syn message to the peer.
    async fn send_init_syn(
        self,
        _state: Self::SendInitSynIn,
    ) -> Result<Self::SendInitSynOut, Self::Error> {
        Ok(PatchType::CURRENT)
    }

    /// Receives and validates the init ack message from the peer.
    async fn recv_init_ack(
        self,
        input: Self::RecvInitAckIn,
    ) -> Result<Self::RecvInitAckOut, Self::Error> {
        let (state, other_ext) = input;
        if other_ext > PatchType::CURRENT {
            bail!(
                "Acceptor patch should be lesser or equal to {current:?}, found {other:?}",
                current = PatchType::CURRENT.raw(),
                other = other_ext.raw(),
            );
        }
        state.patch = other_ext;
        Ok(())
    }

    /// Sends the open syn message to the peer.
    async fn send_open_syn(
        self,
        _state: Self::SendOpenSynIn,
    ) -> Result<Self::SendOpenSynOut, Self::Error> {
        unimplemented!("There is no patch extension in OPEN")
    }

    /// Receives and validates the open ack message from the peer.
    async fn recv_open_ack(
        self,
        _state: Self::RecvOpenAckIn,
    ) -> Result<Self::RecvOpenAckOut, Self::Error> {
        unimplemented!("There is no patch extension in OPEN")
    }
}

/// **********************************
// ACCEPT

/// **********************************
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct StateAccept {
    /// Patch type negotiated during the transport open handshake.
    patch: PatchType,
}

impl StateAccept {
    /// Creates a new state accept with the supplied configuration.
    pub(crate) const fn new() -> Self {
        Self {
            patch: PatchType::NONE,
        }
    }

    /// Returns the value requested from this object.
    pub(crate) const fn get(&self) -> PatchType {
        self.patch
    }

    /// Generates a random value using the supplied random source.
    #[cfg(test)]
    pub(crate) fn rand() -> Self {
        Self {
            patch: PatchType::rand(),
        }
    }
}

// Codec
impl<W> WCodec<&StateAccept, &mut W> for Zenoh080
where
    W: Writer,
{
    /// Alias for `Result<(), DidntWrite>` used to represent output.
    type Output = Result<(), DidntWrite>;

    /// Writes the supplied value to the configured output sink.
    fn write(self, writer: &mut W, x: &StateAccept) -> Self::Output {
        let raw = x.patch.raw();
        self.write(&mut *writer, raw)?;
        Ok(())
    }
}

impl<R> RCodec<StateAccept, &mut R> for Zenoh080
where
    R: Reader,
{
    /// Alias for `DidntRead` used to represent error.
    type Error = DidntRead;

    /// Reads the next value from the supplied input source.
    fn read(self, reader: &mut R) -> Result<StateAccept, Self::Error> {
        let raw: u8 = self.read(&mut *reader)?;
        let patch = PatchType::new(raw);
        Ok(StateAccept { patch })
    }
}

#[async_trait]
impl<'a> AcceptFsm for &'a PatchFsm<'a> {
    /// Alias for `ZError` used to represent error.
    type Error = ZError;
    /// Alias for `(&'a mut StateAccept, PatchType)` used to represent recv init syn in.
    type RecvInitSynIn = (&'a mut StateAccept, PatchType);
    /// Alias for `()` used to represent recv init syn out.
    type RecvInitSynOut = ();
    /// Alias for `(&'a mut StateAccept, ())` used to represent recv open syn in.
    type RecvOpenSynIn = (&'a mut StateAccept, ());
    /// Alias for `()` used to represent recv open syn out.
    type RecvOpenSynOut = ();
    /// Alias for `&'a StateAccept` used to represent send init ack in.
    type SendInitAckIn = &'a StateAccept;
    /// Alias for `PatchType` used to represent send init ack out.
    type SendInitAckOut = PatchType;
    /// Alias for `&'a StateAccept` used to represent send open ack in.
    type SendOpenAckIn = &'a StateAccept;
    /// Alias for `()` used to represent send open ack out.
    type SendOpenAckOut = ();

    /// Receives and validates the init syn message from the peer.
    async fn recv_init_syn(
        self,
        input: Self::RecvInitSynIn,
    ) -> Result<Self::RecvInitSynOut, Self::Error> {
        let (state, other_ext) = input;
        state.patch = other_ext;
        Ok(())
    }

    /// Sends the init ack message to the peer.
    async fn send_init_ack(
        self,
        state: Self::SendInitAckIn,
    ) -> Result<Self::SendInitAckOut, Self::Error> {
        Ok(min(PatchType::CURRENT, state.patch))
    }

    /// Receives and validates the open syn message from the peer.
    async fn recv_open_syn(
        self,
        _state: Self::RecvOpenSynIn,
    ) -> Result<Self::RecvOpenSynOut, Self::Error> {
        unimplemented!("There is no patch extension in OPEN")
    }

    /// Sends the open ack message to the peer.
    async fn send_open_ack(
        self,
        _state: Self::SendOpenAckIn,
    ) -> Result<Self::SendOpenAckOut, Self::Error> {
        unimplemented!("There is no patch extension in OPEN")
    }
}
