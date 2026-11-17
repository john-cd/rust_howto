//! Organizes the auth subsystem and declares its child modules.
//! Parent components use this module to access the subsystem functionality.
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
#[cfg(feature = "auth_pubkey")]
pub(crate) mod pubkey;
/// Provides usrpwd behavior for the auth component.
#[cfg(feature = "auth_usrpwd")]
pub(crate) mod usrpwd;

use std::convert::TryInto;
use std::marker::PhantomData;

use async_trait::async_trait;
#[cfg(feature = "auth_pubkey")]
pub use pubkey::*;
use rand::CryptoRng;
use rand::Rng;
use tokio::sync::Mutex;
use tokio::sync::RwLock;
#[cfg(feature = "auth_usrpwd")]
pub use usrpwd::*;
use zenoh_buffers::ZBuf;
use zenoh_buffers::reader::DidntRead;
use zenoh_buffers::reader::HasReader;
use zenoh_buffers::reader::Reader;
use zenoh_buffers::reader::SiphonableReader;
use zenoh_buffers::writer::DidntWrite;
use zenoh_buffers::writer::HasWriter;
use zenoh_buffers::writer::Writer;
use zenoh_codec::RCodec;
use zenoh_codec::WCodec;
use zenoh_codec::Zenoh080;
use zenoh_config::Config;
use zenoh_core::Error as ZError;
use zenoh_core::Result as ZResult;
use zenoh_core::bail;
use zenoh_core::zerror;
use zenoh_crypto::PseudoRng;
use zenoh_protocol::common::ZExtUnknown;
use zenoh_protocol::common::iext;
use zenoh_protocol::transport::init;
use zenoh_protocol::transport::open;

use crate::unicast::establishment::AcceptFsm;
use crate::unicast::establishment::OpenFsm;

/// Provides id behavior for the auth component.
pub(crate) mod id {
    /// Defines the pubkey value used by this code.
    #[cfg(feature = "auth_pubkey")]
    pub(crate) const PUBKEY: u8 = 0x1;
    /// Defines the usrpwd value used by this code.
    #[cfg(feature = "auth_usrpwd")]
    pub(crate) const USRPWD: u8 = 0x2;
}

/// Data for auth operations, including pubkey and usrpwd.
#[derive(Debug, Default)]
pub struct Auth {
    /// Public key used to authenticate or encrypt data for the peer.
    #[cfg(feature = "auth_pubkey")]
    pubkey: Option<RwLock<AuthPubKey>>,
    /// User name and password credentials used by the authentication extension.
    #[cfg(feature = "auth_usrpwd")]
    usrpwd: Option<RwLock<AuthUsrPwd>>,
}

impl Auth {
    /// Constructs this value from a config.
    pub(crate) async fn from_config(config: &Config) -> ZResult<Self> {
        let auth = config.transport().auth();

        Ok(Self {
            #[cfg(feature = "auth_pubkey")]
            pubkey: AuthPubKey::from_config(auth.pubkey())?.map(RwLock::new),
            #[cfg(feature = "auth_usrpwd")]
            usrpwd: AuthUsrPwd::from_config(auth.usrpwd())
                .await?
                .map(RwLock::new),
        })
    }

    /// Opens the transport using the negotiated session parameters.
    pub(crate) fn open<R>(&self, #[allow(unused)] prng: &mut R) -> StateOpen
    where
        R: Rng + CryptoRng,
    {
        StateOpen {
            #[cfg(feature = "auth_pubkey")]
            pubkey: self.pubkey.is_some().then_some(pubkey::StateOpen::new()),
            #[cfg(feature = "auth_usrpwd")]
            usrpwd: self
                .usrpwd
                .is_some()
                .then_some(usrpwd::StateOpen::new(prng)),
        }
    }

    /// Handles the accept behavior for this component.
    pub(crate) fn accept<R>(&self, #[allow(unused)] prng: &mut R) -> StateAccept
    where
        R: Rng + CryptoRng,
    {
        StateAccept {
            #[cfg(feature = "auth_pubkey")]
            pubkey: self.pubkey.is_some().then_some(pubkey::StateAccept::new()),
            #[cfg(feature = "auth_usrpwd")]
            usrpwd: self
                .usrpwd
                .is_some()
                .then_some(usrpwd::StateAccept::new(prng)),
        }
    }

    /// Handles the fsm behavior for this component.
    pub(crate) fn fsm<'a>(&'a self, #[allow(unused)] prng: &'a Mutex<PseudoRng>) -> AuthFsm<'a> {
        AuthFsm {
            #[cfg(feature = "auth_pubkey")]
            pubkey: self.pubkey.as_ref().map(|x| AuthPubKeyFsm::new(x, prng)),
            #[cfg(feature = "auth_usrpwd")]
            usrpwd: self.usrpwd.as_ref().map(AuthUsrPwdFsm::new),
            _a: PhantomData,
        }
    }
}

#[cfg(feature = "test")]
impl Auth {
    /// Handles the empty behavior for this component.
    pub const fn empty() -> Self {
        Self {
            #[cfg(feature = "auth_pubkey")]
            pubkey: None,
            #[cfg(feature = "auth_usrpwd")]
            usrpwd: None,
        }
    }

    /// Sets the pubkey used by this value.
    #[cfg(feature = "auth_pubkey")]
    pub fn set_pubkey(&mut self, pubkey: Option<AuthPubKey>) {
        self.pubkey = pubkey.map(RwLock::new);
    }

    /// Returns the pubkey stored by or available from this value.
    #[cfg(feature = "auth_pubkey")]
    pub fn get_pubkey(&self) -> Option<&RwLock<AuthPubKey>> {
        self.pubkey.as_ref()
    }

    /// Sets the usrpwd used by this value.
    #[cfg(feature = "auth_usrpwd")]
    pub fn set_usrpwd(&mut self, usrpwd: Option<AuthUsrPwd>) {
        self.usrpwd = usrpwd.map(RwLock::new);
    }

    /// Returns the usrpwd stored by or available from this value.
    #[cfg(feature = "auth_usrpwd")]
    pub fn get_usrpwd(&self) -> Option<&RwLock<AuthUsrPwd>> {
        self.usrpwd.as_ref()
    }
}

/// Data for auth fsm operations, including pubkey and usrpwd.
pub(crate) struct AuthFsm<'a> {
    /// Public key used to authenticate or encrypt data for the peer.
    #[cfg(feature = "auth_pubkey")]
    pubkey: Option<AuthPubKeyFsm<'a>>,
    /// User name and password credentials used by the authentication extension.
    #[cfg(feature = "auth_usrpwd")]
    usrpwd: Option<AuthUsrPwdFsm<'a>>,
    /// a value tracked by this type.
    _a: PhantomData<&'a ()>, // Required only when all auth features are disabled
}

/// Data for state open operations, including pubkey and usrpwd.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct StateOpen {
    /// Public key used to authenticate or encrypt data for the peer.
    #[cfg(feature = "auth_pubkey")]
    pubkey: Option<pubkey::StateOpen>,
    /// User name and password credentials used by the authentication extension.
    #[cfg(feature = "auth_usrpwd")]
    usrpwd: Option<usrpwd::StateOpen>,
}

/// Data for state accept operations, including pubkey and usrpwd.
#[derive(Debug, PartialEq)]
pub(crate) struct StateAccept {
    /// Public key used to authenticate or encrypt data for the peer.
    #[cfg(feature = "auth_pubkey")]
    pubkey: Option<pubkey::StateAccept>,
    /// User name and password credentials used by the authentication extension.
    #[cfg(feature = "auth_usrpwd")]
    usrpwd: Option<usrpwd::StateAccept>,
}

impl StateAccept {
    /// Generates a random value using the supplied random source.
    #[cfg(test)]
    pub(crate) fn rand() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            #[cfg(feature = "auth_pubkey")]
            pubkey: rng.gen_bool(0.5).then_some(pubkey::StateAccept::rand()),
            #[cfg(feature = "auth_usrpwd")]
            usrpwd: rng.gen_bool(0.5).then_some(usrpwd::StateAccept::rand()),
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
        let mut count: usize = 0;
        let mut buff = vec![];
        let mut wbuf = buff.writer();

        #[cfg(feature = "auth_pubkey")]
        {
            if let Some(pubkey) = x.pubkey.as_ref() {
                self.write(&mut wbuf, id::PUBKEY)?;
                self.write(&mut wbuf, pubkey)?;
                count += 1;
            }
        }

        #[cfg(feature = "auth_usrpwd")]
        {
            if let Some(usrpwd) = x.usrpwd.as_ref() {
                self.write(&mut wbuf, id::USRPWD)?;
                self.write(&mut wbuf, usrpwd)?;
                count += 1;
            }
        }

        self.write(&mut *writer, count)?;
        if !buff.is_empty() {
            let mut rbuf = buff.reader();
            rbuf.siphon(&mut *writer).map_err(|_| DidntWrite)?;
        }

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
        let mut count: usize = self.read(&mut *reader)?;

        #[cfg(feature = "auth_pubkey")]
        let mut pubkey: Option<pubkey::StateAccept> = None;
        #[cfg(feature = "auth_usrpwd")]
        let mut usrpwd: Option<usrpwd::StateAccept> = None;

        while count > 0 {
            let e: u8 = self.read(&mut *reader)?;
            match e {
                #[cfg(feature = "auth_pubkey")]
                id::PUBKEY => {
                    pubkey = Some(self.read(&mut *reader)?);
                }
                #[cfg(feature = "auth_usrpwd")]
                id::USRPWD => {
                    usrpwd = Some(self.read(&mut *reader)?);
                }
                _ => return Err(DidntRead),
            }

            count -= 1;
        }

        let state = StateAccept {
            #[cfg(feature = "auth_pubkey")]
            pubkey,
            #[cfg(feature = "auth_usrpwd")]
            usrpwd,
        };
        Ok(state)
    }
}

/// Expands the ztryinto pattern into Rust code at compile time.
macro_rules! ztryinto {
    ($x:expr, $s:expr) => {
        if let Some(x) = $x {
            Some(
                x.try_into()
                    .map_err(|_| zerror!("{} Decoding error.", $s))?,
            )
        } else {
            None
        }
    };
}

/// Expands the ztake pattern into Rust code at compile time.
macro_rules! ztake {
    ($x:expr, $id:expr) => {
        $x.iter()
            .position(|x| x.id & iext::ID_MASK == $id)
            .map(|i| $x.remove(i))
    };
}

/// **********************************
// OPEN

/// **********************************
#[async_trait]
impl<'a> OpenFsm for &'a AuthFsm<'a> {
    /// Alias for `ZError` used to represent error.
    type Error = ZError;
    /// Alias for `(&'a mut StateOpen, Option<init::ext::Auth>)` used to represent recv init ack in.
    type RecvInitAckIn = (&'a mut StateOpen, Option<init::ext::Auth>);
    /// Alias for `()` used to represent recv init ack out.
    type RecvInitAckOut = ();
    /// Alias for `(&'a mut StateOpen, Option<open::ext::Auth>)` used to represent recv open ack in.
    type RecvOpenAckIn = (&'a mut StateOpen, Option<open::ext::Auth>);
    /// Alias for `()` used to represent recv open ack out.
    type RecvOpenAckOut = ();
    /// Alias for `&'a StateOpen` used to represent send init syn in.
    type SendInitSynIn = &'a StateOpen;
    /// Alias for `Option<init::ext::Auth>` used to represent send init syn out.
    type SendInitSynOut = Option<init::ext::Auth>;
    /// Alias for `&'a StateOpen` used to represent send open syn in.
    type SendOpenSynIn = &'a StateOpen;
    /// Alias for `Option<open::ext::Auth>` used to represent send open syn out.
    type SendOpenSynOut = Option<open::ext::Auth>;

    /// Sends the init syn message to the peer.
    async fn send_init_syn(
        self,
        state: Self::SendInitSynIn,
    ) -> Result<Self::SendInitSynOut, Self::Error> {
        /// Defines the s value used by this code.
        const S: &str = "Auth extension - Send InitSyn.";

        let mut exts: Vec<ZExtUnknown> = vec![];

        #[cfg(feature = "auth_pubkey")]
        {
            match (self.pubkey.as_ref(), state.pubkey.as_ref()) {
                (Some(e), Some(s)) => {
                    if let Some(e) = e.send_init_syn(s).await?.take() {
                        exts.push(e.into())
                    }
                }
                (None, None) => {}
                _ => bail!("{S} Invalid PubKey configuration."),
            }
        }

        #[cfg(feature = "auth_usrpwd")]
        {
            match (self.usrpwd.as_ref(), state.usrpwd.as_ref()) {
                (Some(e), Some(s)) => {
                    if let Some(e) = e.send_init_syn(s).await?.take() {
                        exts.push(e.into())
                    }
                }
                (None, None) => {}
                _ => bail!("{S} Invalid UsrPwd configuration."),
            }
        }

        let codec = Zenoh080::new();
        let mut buff = vec![];
        let mut writer = buff.writer();
        codec
            .write(&mut writer, exts.as_slice())
            .map_err(|_| zerror!("{S} Encoding error."))?;

        let output = (!buff.is_empty()).then_some(init::ext::Auth::new(buff.into()));
        Ok(output)
    }

    /// Receives and validates the init ack message from the peer.
    async fn recv_init_ack(
        self,
        input: Self::RecvInitAckIn,
    ) -> Result<Self::RecvInitAckOut, Self::Error> {
        /// Defines the s value used by this code.
        const S: &str = "Auth extension - Recv InitAck.";

        let (state, ext) = input;
        let ext = ext.unwrap_or(init::ext::Auth::new(ZBuf::empty()));

        let codec = Zenoh080::new();
        let mut reader = ext.value.reader();
        let mut exts: Vec<ZExtUnknown> = codec
            .read(&mut reader)
            .map_err(|_| zerror!("{S} Decoding error."))?;

        #[cfg(feature = "auth_pubkey")]
        {
            match (self.pubkey.as_ref(), state.pubkey.as_mut()) {
                (Some(e), Some(s)) => {
                    let x = ztake!(exts, id::PUBKEY);
                    e.recv_init_ack((s, ztryinto!(x, S))).await?;
                }
                (None, None) => {}
                _ => bail!("{S} Invalid PubKey configuration."),
            }
        }

        #[cfg(feature = "auth_usrpwd")]
        {
            match (self.usrpwd.as_ref(), state.usrpwd.as_mut()) {
                (Some(e), Some(s)) => {
                    let x = ztake!(exts, id::USRPWD);
                    e.recv_init_ack((s, ztryinto!(x, S))).await?;
                }
                (None, None) => {}
                _ => bail!("{S} Invalid UsrPwd configuration."),
            }
        }

        Ok(())
    }

    /// Sends the open syn message to the peer.
    async fn send_open_syn(
        self,
        state: Self::SendOpenSynIn,
    ) -> Result<Self::SendOpenSynOut, Self::Error> {
        /// Defines the s value used by this code.
        const S: &str = "Auth extension - Send OpenSyn.";

        let mut exts: Vec<ZExtUnknown> = vec![];

        #[cfg(feature = "auth_pubkey")]
        {
            match (self.pubkey.as_ref(), state.pubkey.as_ref()) {
                (Some(e), Some(s)) => {
                    if let Some(e) = e.send_open_syn(s).await?.take() {
                        exts.push(e.into())
                    }
                }
                (None, None) => {}
                _ => bail!("{S} Invalid PubKey configuration."),
            }
        }

        #[cfg(feature = "auth_usrpwd")]
        {
            match (self.usrpwd.as_ref(), state.usrpwd.as_ref()) {
                (Some(e), Some(s)) => {
                    if let Some(e) = e.send_open_syn(s).await?.take() {
                        exts.push(e.into())
                    }
                }
                (None, None) => {}
                _ => bail!("{S} Invalid UsrPwd configuration."),
            }
        }

        let codec = Zenoh080::new();
        let mut buff = vec![];
        let mut writer = buff.writer();
        codec
            .write(&mut writer, exts.as_slice())
            .map_err(|_| zerror!("{S} Encoding error."))?;

        let output = (!buff.is_empty()).then_some(open::ext::Auth::new(buff.into()));
        Ok(output)
    }

    /// Receives and validates the open ack message from the peer.
    async fn recv_open_ack(
        self,
        input: Self::RecvOpenAckIn,
    ) -> Result<Self::RecvOpenAckOut, Self::Error> {
        /// Defines the s value used by this code.
        const S: &str = "Auth extension - Recv OpenAck.";

        let (state, ext) = input;
        let ext = ext.unwrap_or(init::ext::Auth::new(ZBuf::empty()));

        let codec = Zenoh080::new();
        let mut reader = ext.value.reader();
        let mut exts: Vec<ZExtUnknown> = codec
            .read(&mut reader)
            .map_err(|_| zerror!("{S} Decoding error."))?;

        #[cfg(feature = "auth_pubkey")]
        {
            match (self.pubkey.as_ref(), state.pubkey.as_mut()) {
                (Some(e), Some(s)) => {
                    let x = ztake!(exts, id::PUBKEY);
                    e.recv_open_ack((s, ztryinto!(x, S))).await?;
                }
                (None, None) => {}
                _ => bail!("{S} Invalid PubKey configuration."),
            }
        }

        #[cfg(feature = "auth_usrpwd")]
        {
            match (self.usrpwd.as_ref(), state.usrpwd.as_mut()) {
                (Some(e), Some(s)) => {
                    let x = ztake!(exts, id::USRPWD);
                    e.recv_open_ack((s, ztryinto!(x, S))).await?;
                }
                (None, None) => {}
                _ => bail!("{S} Invalid UsrPwd configuration."),
            }
        }

        Ok(())
    }
}

/// **********************************
// ACCEPT

/// **********************************
pub(crate) struct RecvOpenSynOut {
    /// Identifier of the authenticated peer.
    #[cfg(feature = "auth_usrpwd")]
    pub(crate) auth_id: UsrPwdId,
}

#[async_trait]
impl<'a> AcceptFsm for &'a AuthFsm<'a> {
    /// Alias for `ZError` used to represent error.
    type Error = ZError;
    /// Alias for `(&'a mut StateAccept, Option<init::ext::Auth>)` used to represent recv init syn
    /// in.
    type RecvInitSynIn = (&'a mut StateAccept, Option<init::ext::Auth>);
    /// Alias for `()` used to represent recv init syn out.
    type RecvInitSynOut = ();
    /// Alias for `(&'a mut StateAccept, Option<open::ext::Auth>)` used to represent recv open syn
    /// in.
    type RecvOpenSynIn = (&'a mut StateAccept, Option<open::ext::Auth>);
    /// Alias for `RecvOpenSynOut` used to represent recv open syn out.
    type RecvOpenSynOut = RecvOpenSynOut;
    /// Alias for `&'a StateAccept` used to represent send init ack in.
    type SendInitAckIn = &'a StateAccept;
    /// Alias for `Option<init::ext::Auth>` used to represent send init ack out.
    type SendInitAckOut = Option<init::ext::Auth>;
    /// Alias for `&'a StateAccept` used to represent send open ack in.
    type SendOpenAckIn = &'a StateAccept;
    /// Alias for `Option<open::ext::Auth>` used to represent send open ack out.
    type SendOpenAckOut = Option<open::ext::Auth>;

    /// Receives and validates the init syn message from the peer.
    async fn recv_init_syn(
        self,
        input: Self::RecvInitSynIn,
    ) -> Result<Self::RecvInitSynOut, Self::Error> {
        /// Defines the s value used by this code.
        const S: &str = "Auth extension - Recv InitSyn.";

        let (state, ext) = input;
        let ext = ext.unwrap_or(init::ext::Auth::new(ZBuf::empty()));

        let codec = Zenoh080::new();
        let mut reader = ext.value.reader();
        let mut exts: Vec<ZExtUnknown> = codec
            .read(&mut reader)
            .map_err(|_| zerror!("{S} Decoding error."))?;

        #[cfg(feature = "auth_pubkey")]
        {
            match (self.pubkey.as_ref(), state.pubkey.as_mut()) {
                (Some(e), Some(s)) => {
                    let x = ztake!(exts, id::PUBKEY);
                    e.recv_init_syn((s, ztryinto!(x, S))).await?;
                }
                (None, None) => {}
                _ => bail!("{S} Invalid PubKey configuration."),
            }
        }

        #[cfg(feature = "auth_usrpwd")]
        {
            match (self.usrpwd.as_ref(), state.usrpwd.as_mut()) {
                (Some(e), Some(s)) => {
                    let x = ztake!(exts, id::USRPWD);
                    e.recv_init_syn((s, ztryinto!(x, S))).await?;
                }
                (None, None) => {}
                _ => bail!("{S} Invalid UsrPwd configuration."),
            }
        }

        Ok(())
    }

    /// Sends the init ack message to the peer.
    async fn send_init_ack(
        self,
        state: Self::SendInitAckIn,
    ) -> Result<Self::SendInitAckOut, Self::Error> {
        /// Defines the s value used by this code.
        const S: &str = "Auth extension - Send InitAck.";

        let mut exts: Vec<ZExtUnknown> = vec![];

        #[cfg(feature = "auth_pubkey")]
        {
            match (self.pubkey.as_ref(), state.pubkey.as_ref()) {
                (Some(e), Some(s)) => {
                    if let Some(e) = e.send_init_ack(s).await?.take() {
                        exts.push(e.into())
                    }
                }
                (None, None) => {}
                _ => bail!("{S} Invalid PubKey configuration."),
            }
        }

        #[cfg(feature = "auth_usrpwd")]
        {
            match (self.usrpwd.as_ref(), state.usrpwd.as_ref()) {
                (Some(e), Some(s)) => {
                    if let Some(e) = e.send_init_ack(s).await?.take() {
                        exts.push(e.into())
                    }
                }
                (None, None) => {}
                _ => bail!("{S} Invalid UsrPwd configuration."),
            }
        }

        let codec = Zenoh080::new();
        let mut buff = vec![];
        let mut writer = buff.writer();
        codec
            .write(&mut writer, exts.as_slice())
            .map_err(|_| zerror!("{S} Encoding error."))?;

        let output = (!buff.is_empty()).then_some(init::ext::Auth::new(buff.into()));
        Ok(output)
    }

    /// Receives and validates the open syn message from the peer.
    async fn recv_open_syn(
        self,
        input: Self::RecvOpenSynIn,
    ) -> Result<Self::RecvOpenSynOut, Self::Error> {
        /// Defines the s value used by this code.
        const S: &str = "Auth extension - Recv OpenSyn.";

        let (state, ext) = input;
        let ext = ext.unwrap_or(init::ext::Auth::new(ZBuf::empty()));

        let codec = Zenoh080::new();
        let mut reader = ext.value.reader();
        let mut exts: Vec<ZExtUnknown> = codec
            .read(&mut reader)
            .map_err(|_| zerror!("{S} Decoding error."))?;

        #[cfg(feature = "auth_pubkey")]
        {
            match (self.pubkey.as_ref(), state.pubkey.as_mut()) {
                (Some(e), Some(s)) => {
                    let x = ztake!(exts, id::PUBKEY);
                    e.recv_open_syn((s, ztryinto!(x, S))).await?;
                }
                (None, None) => {}
                _ => bail!("{S} Invalid PubKey configuration."),
            }
        }

        #[cfg(feature = "auth_usrpwd")]
        let auth_id: UsrPwdId;

        #[cfg(feature = "auth_usrpwd")]
        {
            match (self.usrpwd.as_ref(), state.usrpwd.as_mut()) {
                (Some(e), Some(s)) => {
                    let x = ztake!(exts, id::USRPWD);
                    let username = e.recv_open_syn((s, ztryinto!(x, S))).await?;
                    auth_id = UsrPwdId(Some(username));
                }
                (None, None) => {
                    auth_id = UsrPwdId(None);
                }
                _ => bail!("{S} Invalid UsrPwd configuration."),
            }
        }
        Ok(RecvOpenSynOut {
            #[cfg(feature = "auth_usrpwd")]
            auth_id,
        })
    }

    /// Sends the open ack message to the peer.
    async fn send_open_ack(
        self,
        state: Self::SendOpenAckIn,
    ) -> Result<Self::SendOpenAckOut, Self::Error> {
        /// Defines the s value used by this code.
        const S: &str = "Auth extension - Send OpenAck.";

        let mut exts: Vec<ZExtUnknown> = vec![];

        #[cfg(feature = "auth_pubkey")]
        {
            match (self.pubkey.as_ref(), state.pubkey.as_ref()) {
                (Some(e), Some(s)) => {
                    if let Some(e) = e.send_open_ack(s).await?.take() {
                        exts.push(e.into())
                    }
                }
                (None, None) => {}
                _ => bail!("{S} Invalid PubKey configuration."),
            }
        }

        #[cfg(feature = "auth_usrpwd")]
        {
            match (self.usrpwd.as_ref(), state.usrpwd.as_ref()) {
                (Some(e), Some(s)) => {
                    if let Some(e) = e.send_open_ack(s).await?.take() {
                        exts.push(e.into())
                    }
                }
                (None, None) => {}
                _ => bail!("{S} Invalid UsrPwd configuration."),
            }
        }

        let codec = Zenoh080::new();
        let mut buff = vec![];
        let mut writer = buff.writer();
        codec
            .write(&mut writer, exts.as_slice())
            .map_err(|_| zerror!("{S} Encoding error."))?;

        let output = (!buff.is_empty()).then_some(open::ext::Auth::new(buff.into()));
        Ok(output)
    }
}
