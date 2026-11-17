//! Negotiates compression settings for a unicast transport.
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
use core::marker::PhantomData;

use async_trait::async_trait;
use zenoh_buffers::reader::DidntRead;
use zenoh_buffers::reader::Reader;
use zenoh_buffers::writer::DidntWrite;
use zenoh_buffers::writer::Writer;
use zenoh_codec::RCodec;
use zenoh_codec::WCodec;
use zenoh_codec::Zenoh080;
use zenoh_protocol::transport::init;
use zenoh_protocol::transport::open;
use zenoh_result::Error as ZError;

use crate::unicast::establishment::AcceptFsm;
use crate::unicast::establishment::OpenFsm;

// Extension Fsm
pub(crate) struct CompressionFsm<'a> {
    /// a value tracked by this type.
    _a: PhantomData<&'a ()>,
}

impl CompressionFsm<'_> {
    /// Creates a new compression fsm with the supplied configuration.
    pub(crate) const fn new() -> Self {
        Self { _a: PhantomData }
    }
}

/// **********************************
// OPEN

/// **********************************
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct StateOpen {
    /// Whether compression was negotiated for the session.
    is_compression: bool,
}

impl StateOpen {
    /// Creates a new state open with the supplied configuration.
    pub(crate) const fn new(is_compression: bool) -> Self {
        Self { is_compression }
    }

    /// Reports whether this value is compression.
    pub(crate) const fn is_compression(&self) -> bool {
        self.is_compression
    }
}

#[async_trait]
impl<'a> OpenFsm for &'a CompressionFsm<'a> {
    /// Alias for `ZError` used to represent error.
    type Error = ZError;
    /// Alias for `(&'a mut StateOpen, Option<init::ext::Compression>)` used to represent recv init
    /// ack in.
    type RecvInitAckIn = (&'a mut StateOpen, Option<init::ext::Compression>);
    /// Alias for `()` used to represent recv init ack out.
    type RecvInitAckOut = ();
    /// Alias for `(&'a mut StateOpen, Option<open::ext::Compression>)` used to represent recv open
    /// ack in.
    type RecvOpenAckIn = (&'a mut StateOpen, Option<open::ext::Compression>);
    /// Alias for `()` used to represent recv open ack out.
    type RecvOpenAckOut = ();
    /// Alias for `&'a StateOpen` used to represent send init syn in.
    type SendInitSynIn = &'a StateOpen;
    /// Alias for `Option<init::ext::Compression>` used to represent send init syn out.
    type SendInitSynOut = Option<init::ext::Compression>;
    /// Alias for `&'a StateOpen` used to represent send open syn in.
    type SendOpenSynIn = &'a StateOpen;
    /// Alias for `Option<open::ext::Compression>` used to represent send open syn out.
    type SendOpenSynOut = Option<open::ext::Compression>;

    /// Sends the init syn message to the peer.
    async fn send_init_syn(
        self,
        state: Self::SendInitSynIn,
    ) -> Result<Self::SendInitSynOut, Self::Error> {
        let output = state
            .is_compression
            .then_some(init::ext::Compression::new());
        Ok(output)
    }

    /// Receives and validates the init ack message from the peer.
    async fn recv_init_ack(
        self,
        input: Self::RecvInitAckIn,
    ) -> Result<Self::RecvInitAckOut, Self::Error> {
        let (state, other_ext) = input;
        state.is_compression &= other_ext.is_some();
        Ok(())
    }

    /// Sends the open syn message to the peer.
    async fn send_open_syn(
        self,
        _state: Self::SendOpenSynIn,
    ) -> Result<Self::SendOpenSynOut, Self::Error> {
        Ok(None)
    }

    /// Receives and validates the open ack message from the peer.
    async fn recv_open_ack(
        self,
        _state: Self::RecvOpenAckIn,
    ) -> Result<Self::RecvOpenAckOut, Self::Error> {
        Ok(())
    }
}

/// **********************************
// ACCEPT

/// **********************************
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct StateAccept {
    /// Whether compression was negotiated for the session.
    is_compression: bool,
}

impl StateAccept {
    /// Creates a new state accept with the supplied configuration.
    pub(crate) const fn new(is_compression: bool) -> Self {
        Self { is_compression }
    }

    /// Reports whether this value is compression.
    pub(crate) const fn is_compression(&self) -> bool {
        self.is_compression
    }

    /// Generates a random value using the supplied random source.
    #[cfg(test)]
    pub(crate) fn rand() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        Self::new(rng.gen_bool(0.5))
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
        let is_compression = u8::from(x.is_compression);
        self.write(&mut *writer, is_compression)?;
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
        let is_compression: u8 = self.read(&mut *reader)?;
        let is_compression = is_compression == 1;
        Ok(StateAccept { is_compression })
    }
}

#[async_trait]
impl<'a> AcceptFsm for &'a CompressionFsm<'a> {
    /// Alias for `ZError` used to represent error.
    type Error = ZError;
    /// Alias for `(&'a mut StateAccept, Option<init::ext::Compression>)` used to represent recv
    /// init syn in.
    type RecvInitSynIn = (&'a mut StateAccept, Option<init::ext::Compression>);
    /// Alias for `()` used to represent recv init syn out.
    type RecvInitSynOut = ();
    /// Alias for `(&'a mut StateAccept, Option<open::ext::Compression>)` used to represent recv
    /// open syn in.
    type RecvOpenSynIn = (&'a mut StateAccept, Option<open::ext::Compression>);
    /// Alias for `()` used to represent recv open syn out.
    type RecvOpenSynOut = ();
    /// Alias for `&'a StateAccept` used to represent send init ack in.
    type SendInitAckIn = &'a StateAccept;
    /// Alias for `Option<init::ext::Compression>` used to represent send init ack out.
    type SendInitAckOut = Option<init::ext::Compression>;
    /// Alias for `&'a StateAccept` used to represent send open ack in.
    type SendOpenAckIn = &'a StateAccept;
    /// Alias for `Option<open::ext::Compression>` used to represent send open ack out.
    type SendOpenAckOut = Option<open::ext::Compression>;

    /// Receives and validates the init syn message from the peer.
    async fn recv_init_syn(
        self,
        input: Self::RecvInitSynIn,
    ) -> Result<Self::RecvInitSynOut, Self::Error> {
        let (state, other_ext) = input;
        state.is_compression &= other_ext.is_some();
        Ok(())
    }

    /// Sends the init ack message to the peer.
    async fn send_init_ack(
        self,
        state: Self::SendInitAckIn,
    ) -> Result<Self::SendInitAckOut, Self::Error> {
        let output = state
            .is_compression
            .then_some(init::ext::Compression::new());
        Ok(output)
    }

    /// Receives and validates the open syn message from the peer.
    async fn recv_open_syn(
        self,
        _state: Self::RecvOpenSynIn,
    ) -> Result<Self::RecvOpenSynOut, Self::Error> {
        Ok(())
    }

    /// Sends the open ack message to the peer.
    async fn send_open_ack(
        self,
        _state: Self::SendOpenAckIn,
    ) -> Result<Self::SendOpenAckOut, Self::Error> {
        Ok(None)
    }
}
