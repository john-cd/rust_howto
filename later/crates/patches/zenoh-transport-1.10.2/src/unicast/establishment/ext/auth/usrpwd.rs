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

//! Implements username/password challenge-response authentication for unicast peers.
//!
//! The initiator proves knowledge of its password with an HMAC over the received
//! nonce; the acceptor checks that proof against configured credentials.
//!
//! TODO Reviewed ??-??-2026.

use std::collections::HashMap;
use std::fmt;

use async_trait::async_trait;
use rand::CryptoRng;
use rand::Rng;
use tokio::sync::RwLock;
use zenoh_buffers::reader::DidntRead;
use zenoh_buffers::reader::HasReader;
use zenoh_buffers::reader::Reader;
use zenoh_buffers::writer::DidntWrite;
use zenoh_buffers::writer::HasWriter;
use zenoh_buffers::writer::Writer;
use zenoh_codec::RCodec;
use zenoh_codec::WCodec;
use zenoh_codec::Zenoh080;
use zenoh_config::UsrPwdConf;
use zenoh_core::Error as ZError;
use zenoh_core::Result as ZResult;
use zenoh_core::bail;
use zenoh_core::zasyncread;
use zenoh_core::zerror;
use zenoh_crypto::hmac;
use zenoh_protocol::common::ZExtUnit;
use zenoh_protocol::common::ZExtZ64;
use zenoh_protocol::common::ZExtZBuf;

use crate::unicast::establishment::AcceptFsm;
use crate::unicast::establishment::OpenFsm;
use crate::unicast::establishment::ext::auth::id;

/// Wire extension payloads used by the username/password handshake.
mod ext {
    use zenoh_protocol::zextunit;
    use zenoh_protocol::zextz64;
    use zenoh_protocol::zextzbuf;

    use super::id::USRPWD;

    /// Initial request sent by the authentication initiator.
    pub(super) type InitSyn = zextunit!(USRPWD, false);

    /// Nonce challenge sent by the authentication acceptor.
    pub(super) type InitAck = zextz64!(USRPWD, false);

    /// Username and password proof sent by the initiator.
    pub(super) type OpenSyn = zextzbuf!(USRPWD, false);

    /// Acknowledgement sent after the credentials are accepted.
    pub(super) type OpenAck = zextunit!(USRPWD, false);
}

/// Username bytes used in the authentication lookup and handshake.
type User = Vec<u8>;

/// Password bytes stored for a user or supplied as local credentials.
type Password = Vec<u8>;

/// Configured credentials and user dictionary for unicast authentication.
pub struct AuthUsrPwd {

    /// Maps accepted usernames to their password bytes.
    lookup: HashMap<User, Password>,

    /// Local credentials used when this peer initiates a connection.
    credentials: Option<(User, Password)>,
}

impl AuthUsrPwd {

    /// Creates authentication state with the supplied local credentials.
    ///
    /// # Arguments
///
    /// * `credentials` - Optional username and password used when opening a connection.
    ///
    /// # Returns
///
    /// Authentication state with an empty accepted-user dictionary.
    pub fn new(credentials: Option<(User, Password)>) -> Self {
        Self {
            lookup: HashMap::new(),
            credentials,
        }
    }

    /// Adds or replaces a username and password accepted by this authenticator.
    ///
    /// # Arguments
///
    /// * `user` - Username bytes to accept.
    /// * `password` - Password bytes associated with `user`.
    ///
    /// # Returns
///
    /// `Ok(())` after the credential is inserted.
    ///
    /// # Errors
///
    /// This implementation currently has no failure path and returns `Ok(())`.
    pub async fn add_user(&mut self, user: User, password: Password) -> ZResult<()> {
        self.lookup.insert(user, password);
        Ok(())
    }

    /// Removes a username from the accepted-user dictionary if it exists.
    ///
    /// # Arguments
///
    /// * `user` - Username bytes to remove.
    ///
    /// # Returns
///
    /// `Ok(())` whether or not the username was present.
    ///
    /// # Errors
///
    /// This implementation currently has no failure path and returns `Ok(())`.
    pub async fn del_user(&mut self, user: &User) -> ZResult<()> {
        self.lookup.remove(user);
        Ok(())
    }

    /// Loads credentials and an optional user dictionary from configuration.
    ///
    /// # Arguments
///
    /// * `config` - Username, password, and dictionary-file settings.
    ///
    /// # Returns
///
    /// `Some` when local credentials or dictionary users enable authentication;
    /// otherwise `None`.
    ///
    /// # Errors
///
    /// Returns an error if the dictionary cannot be read, or contains a malformed
    /// line, an empty username, or an empty password.
    pub async fn from_config(config: &UsrPwdConf) -> ZResult<Option<Self>> {
        const S: &str = "UsrPwd extension - From config.";

        let mut lookup: HashMap<User, Password> = HashMap::new();
        if let Some(dict) = config.dictionary_file() {
            let content = tokio::fs::read_to_string(dict)
                .await
                .map_err(|e| zerror!("{S} Invalid user-password dictionary file: {}.", e))?;

            // Populate the user-password dictionary
            // The config file is expected to be in the form of:
            //      usr1:pwd1
            //      usr2:pwd2
            //      usr3:pwd3
            // I.e.: one <user>:<password> entry per line
            for l in content.lines() {
                let line = l.trim();
                if line.is_empty() {
                    continue;
                }
                let idx = line.find(':').ok_or_else(|| {
                    zerror!("{S} Invalid user-password dictionary file: invalid format.")
                })?;
                let user = line[..idx].trim().as_bytes().to_owned();
                if user.is_empty() {
                    bail!("{S} Invalid user-password dictionary file: empty user.")
                }
                let password = line[idx + 1..].trim().as_bytes().to_owned();
                if password.is_empty() {
                    bail!("{S} Invalid user-password dictionary file: empty password.")
                }
                lookup.insert(user, password);
            }
            tracing::debug!("{S} User-password dictionary has been configured.");
        }

        let mut credentials: Option<(User, Password)> = None;
        if let Some(user) = config.user() {
            if let Some(password) = config.password() {
                tracing::debug!("{S} User-password has been configured.");
                credentials = Some((user.as_bytes().to_owned(), password.as_bytes().to_owned()));
            }
        }

        if !lookup.is_empty() || credentials.is_some() {
            tracing::debug!("{S} User-password authentication is enabled.");
            Ok(Some(Self {
                lookup,
                credentials,
            }))
        } else {
            Ok(None)
        }
    }
}

impl fmt::Debug for AuthUsrPwd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.credentials.as_ref() {
            Some(c) => write!(
                f,
                "User: '{}', Password: '***', ",
                String::from_utf8_lossy(&c.0)
            )?,
            None => write!(f, "User: '', Password: '', ")?,
        }
        write!(f, "Dictionary: {{")?;
        for (i, (u, _)) in self.lookup.iter().enumerate() {
            if i != 0 {
                write!(f, ",")?;
            }
            write!(f, " {}", String::from_utf8_lossy(u))?;
        }
        write!(f, " }}")
    }
}

// OpenFsm / AcceptFsm

/// Nonce retained by the peer opening a unicast connection.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct StateOpen {

    /// Challenge received from the accepting peer.
    nonce: u64,
}

impl StateOpen {

    /// Creates initiator state with a cryptographically random nonce.
    ///
    /// # Arguments
///
    /// * `prng` - Cryptographic random number generator used for the nonce.
    ///
    /// # Returns
///
    /// State containing a newly generated nonce.
    pub(crate) fn new<R>(prng: &mut R) -> Self
    where
        R: Rng + CryptoRng,
    {
        Self {
            nonce: prng.r#gen(),
        }
    }
}

/// Nonce retained by the peer accepting a unicast connection.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct StateAccept {

    /// Challenge sent to the initiating peer.
    nonce: u64,
}

/// Optional username returned after a successful username/password handshake.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct UsrPwdId(

    /// Authenticated username bytes, or `None` when the extension is absent.
    pub Option<Vec<u8>>,
);

impl StateAccept {

    /// Creates acceptor state with a cryptographically random nonce.
    ///
    /// # Arguments
///
    /// * `prng` - Cryptographic random number generator used for the nonce.
    ///
    /// # Returns
///
    /// State containing a newly generated nonce.
    pub(crate) fn new<R>(prng: &mut R) -> Self
    where
        R: Rng + CryptoRng,
    {
        Self {
            nonce: prng.r#gen(),
        }
    }

    /// Creates acceptor state with the thread-local random generator for tests.
    ///
    /// # Returns
///
    /// State containing a newly generated nonce.
    #[cfg(all(test, feature = "test"))]
    pub(crate) fn rand() -> Self {
        let mut rng = rand::thread_rng();
        Self::new(&mut rng)
    }
}

// Codec

/// Encodes the acceptor's nonce with the Zenoh 8.0 codec.
impl<W> WCodec<&StateAccept, &mut W> for Zenoh080
where
    W: Writer,
{
    type Output = Result<(), DidntWrite>;

    /// Writes the nonce to the provided writer.
    ///
    /// # Arguments
///
    /// * `writer` - Destination writer for the encoded nonce.
    /// * `x` - Acceptor state containing the nonce.
    ///
    /// # Returns
///
    /// The codec write result.
    fn write(self, writer: &mut W, x: &StateAccept) -> Self::Output {
        self.write(&mut *writer, x.nonce)
    }
}

/// Decodes the acceptor's nonce with the Zenoh 8.0 codec.
impl<R> RCodec<StateAccept, &mut R> for Zenoh080
where
    R: Reader,
{
    type Error = DidntRead;

    /// Reads a nonce and constructs acceptor state from it.
    ///
    /// # Arguments
///
    /// * `reader` - Source of the encoded nonce.
    ///
    /// # Returns
///
    /// State containing the decoded nonce.
    ///
    /// # Errors
///
    /// Returns the reader error if the nonce cannot be decoded.
    fn read(self, reader: &mut R) -> Result<StateAccept, Self::Error> {
        let nonce: u64 = self.read(&mut *reader)?;
        Ok(StateAccept { nonce })
    }
}

/// Shared authenticator state used by the open and accept handshake machines.
pub(crate) struct AuthUsrPwdFsm<'a> {

    /// Locked credential configuration shared across handshake steps.
    inner: &'a RwLock<AuthUsrPwd>,
}

impl<'a> AuthUsrPwdFsm<'a> {

    /// Creates a handshake adapter over shared username/password state.
    ///
    /// # Arguments
///
    /// * `inner` - Shared authenticator configuration.
    ///
    /// # Returns
///
    /// An adapter implementing the initiator and acceptor state-machine traits.
    pub(super) const fn new(inner: &'a RwLock<AuthUsrPwd>) -> Self {
        Self { inner }
    }
}

/// **********************************
// OpenSyn

/// **********************************
/// ```text
///  7 6 5 4 3 2 1 0
/// +-+-+-+-+-+-+-+-+
/// ~     user      ~
/// +---------------+
/// ~     hash      ~
/// +---------------+
///
/// ZExtZBuf
/// ```
/// Username and challenge response encoded in the open handshake message.
struct OpenSyn {

    /// Username whose password proof is being checked.
    user: Vec<u8>,

    /// HMAC of the user's password keyed by the challenge nonce.
    hmac: Vec<u8>,
}

/// Encodes the username and HMAC carried by an open request.
impl<W> WCodec<&OpenSyn, &mut W> for Zenoh080
where
    W: Writer,
{
    type Output = Result<(), DidntWrite>;

    /// Writes both fields in wire order.
    ///
    /// # Arguments
///
    /// * `writer` - Destination for the encoded fields.
    /// * `x` - Username and HMAC payload to encode.
    ///
    /// # Returns
///
    /// The codec write result.
    fn write(self, writer: &mut W, x: &OpenSyn) -> Self::Output {
        self.write(&mut *writer, x.user.as_slice())?;
        self.write(&mut *writer, x.hmac.as_slice())?;
        Ok(())
    }
}

/// Decodes the username and HMAC carried by an open request.
impl<R> RCodec<OpenSyn, &mut R> for Zenoh080
where
    R: Reader,
{
    type Error = DidntRead;

    /// Reads both fields in wire order.
    ///
    /// # Arguments
///
    /// * `reader` - Source of the encoded fields.
    ///
    /// # Returns
///
    /// The decoded username and HMAC payload.
    ///
    /// # Errors
///
    /// Returns the reader error if either field cannot be decoded.
    fn read(self, reader: &mut R) -> Result<OpenSyn, Self::Error> {
        let user: Vec<u8> = self.read(&mut *reader)?;
        let hmac: Vec<u8> = self.read(&mut *reader)?;
        Ok(OpenSyn { user, hmac })
    }
}

/// **********************************
// OpenAck

/// **********************************
/// ```text
///  7 6 5 4 3 2 1 0
/// +-+-+-+-+-+-+-+-+
/// +---------------+
///
/// ZExtUnit
/// ```
#[async_trait]
impl<'a> OpenFsm for &'a AuthUsrPwdFsm<'a> {
    type Error = ZError;
    type RecvInitAckIn = (&'a mut StateOpen, Option<ext::InitAck>);
    type RecvInitAckOut = ();
    type RecvOpenAckIn = (&'a mut StateOpen, Option<ext::OpenAck>);
    type RecvOpenAckOut = ();
    type SendInitSynIn = &'a StateOpen;
    type SendInitSynOut = Option<ext::InitSyn>;
    type SendOpenSynIn = &'a StateOpen;
    type SendOpenSynOut = Option<ext::OpenSyn>;

    /// Announces that the initiator supports username/password authentication.
    ///
    /// # Arguments
///
    /// * `_input` - Initiator nonce state, unused until the challenge arrives.
    ///
    /// # Returns
///
    /// An initial authentication extension when local credentials are configured.
    async fn send_init_syn(
        self,
        _input: Self::SendInitSynIn,
    ) -> Result<Self::SendInitSynOut, Self::Error> {
        let output = zasyncread!(self.inner)
            .credentials
            .is_some()
            .then_some(ZExtUnit::new());
        Ok(output)
    }

    /// Stores the nonce challenge received from the acceptor.
    ///
    /// # Arguments
///
    /// * `input` - Initiator state and optional decoded challenge.
    ///
    /// # Errors
///
    /// Returns an error when credentials are configured but the challenge is absent.
    async fn recv_init_ack(
        self,
        input: Self::RecvInitAckIn,
    ) -> Result<Self::RecvInitAckOut, Self::Error> {
        const S: &str = "UsrPwd extension - Recv InitSyn.";

        if zasyncread!(self.inner).credentials.is_none() {
            return Ok(());
        };

        let (state, mut ext_userwpd) = input;
        let ext_usrpwd = ext_userwpd
            .take()
            .ok_or_else(|| zerror!("{S} Decoding error."))?;
        state.nonce = ext_usrpwd.value;

        Ok(())
    }

    /// Encodes the configured username and its nonce-based password proof.
    ///
    /// # Arguments
///
    /// * `state` - Initiator state containing the challenge nonce.
    ///
    /// # Returns
///
    /// An open-request extension when credentials are configured.
    ///
    /// # Errors
///
    /// Returns an error if HMAC generation or extension encoding fails.
    async fn send_open_syn(
        self,
        state: Self::SendOpenSynIn,
    ) -> Result<Self::SendOpenSynOut, Self::Error> {
        const S: &str = "UsrPwd extension - Send OpenSyn.";

        // If credentials are not configured, don't continue the USRPWD authentication
        let r_inner = zasyncread!(self.inner);
        let (user, password) = match r_inner.credentials.as_ref() {
            Some(cr) => cr,
            None => return Ok(None),
        };

        // Create the HMAC of the password using the nonce received as a key (it's a challenge)
        let key = state.nonce.to_le_bytes();
        let hmac = hmac::sign(&key, password).map_err(|_| zerror!("{S} Encoding error."))?;
        // Create the OpenSyn extension
        let open_syn = OpenSyn {
            user: user.to_vec(),
            hmac,
        };
        drop(r_inner);

        let codec = Zenoh080::new();
        let mut buff = vec![];
        let mut writer = buff.writer();
        codec
            .write(&mut writer, &open_syn)
            .map_err(|_| zerror!("{S} Encoding error."))?;

        let output = Some(ZExtZBuf::new(buff.into()));
        Ok(output)
    }

    /// Checks that the acceptor returned the expected authentication response.
    ///
    /// # Arguments
///
    /// * `input` - Initiator state and optional decoded response extension.
    ///
    /// # Errors
///
    /// Returns an error if configured authentication has no response extension.
    async fn recv_open_ack(
        self,
        input: Self::RecvOpenAckIn,
    ) -> Result<Self::RecvOpenAckOut, Self::Error> {
        const S: &str = "UsrPwd extension - Recv OpenAck.";

        let (_, ext) = input;
        if zasyncread!(self.inner).credentials.is_some() && ext.is_none() {
            bail!("{S} Expected extension.");
        }

        Ok(())
    }
}

/// **********************************
// ACCEPT

/// **********************************
#[async_trait]
impl<'a> AcceptFsm for &'a AuthUsrPwdFsm<'a> {
    type Error = ZError;
    type RecvInitSynIn = (&'a mut StateAccept, Option<ext::InitSyn>);
    type RecvInitSynOut = ();
    type RecvOpenSynIn = (&'a mut StateAccept, Option<ext::OpenSyn>);
    type RecvOpenSynOut = Vec<u8>;
    type SendInitAckIn = &'a StateAccept;
    type SendInitAckOut = Option<ext::InitAck>;
    type SendOpenAckIn = &'a StateAccept;
    type SendOpenAckOut = Option<ext::OpenAck>;

    /// Requires an initial authentication request from the initiator.
    ///
    /// # Arguments
///
    /// * `input` - Acceptor state and optional decoded request extension.
    ///
    /// # Errors
///
    /// Returns an error if the request extension is absent.
    async fn recv_init_syn(
        self,
        input: Self::RecvInitSynIn,
    ) -> Result<Self::RecvInitSynOut, Self::Error> {
        const S: &str = "UsrPwd extension - Recv InitSyn.";

        let (_, ext_usrpwd) = input;
        if ext_usrpwd.is_none() {
            bail!("{S} Expected extension.");
        }

        Ok(())
    }

    /// Sends the acceptor's nonce challenge to the initiator.
    ///
    /// # Arguments
///
    /// * `state` - Acceptor state containing the challenge nonce.
    ///
    /// # Returns
///
    /// The encoded nonce challenge extension.
    async fn send_init_ack(
        self,
        state: Self::SendInitAckIn,
    ) -> Result<Self::SendInitAckOut, Self::Error> {
        Ok(Some(ZExtZ64::new(state.nonce)))
    }

    // value of userid is returned if recvopensynout is processed as valid

    /// Verifies the initiator's HMAC and returns the authenticated username.
    ///
    /// # Arguments
///
    /// * `input` - Acceptor state and optional decoded open-request extension.
    ///
    /// # Returns
///
    /// The username whose password proof matched the configured dictionary.
    ///
    /// # Errors
///
    /// Returns an error if the extension is missing or malformed, the user is
    /// unknown, or the password proof is invalid.
    async fn recv_open_syn(
        self,
        input: Self::RecvOpenSynIn,
    ) -> Result<Self::RecvOpenSynOut, Self::Error> {
        const S: &str = "UsrPwd extension - Recv OpenSyn.";

        let (state, mut ext_usrpwd) = input;
        let ext_usrpwd = ext_usrpwd
            .take()
            .ok_or_else(|| zerror!("{S} Expected extension."))?;

        let codec = Zenoh080::new();
        let mut reader = ext_usrpwd.value.reader();
        let open_syn: OpenSyn = codec
            .read(&mut reader)
            .map_err(|_| zerror!("{S} Decoding error."))?;

        let r_inner = zasyncread!(self.inner);
        let pwd = r_inner
            .lookup
            .get(&open_syn.user)
            .ok_or_else(|| zerror!("{S} Invalid user."))?;

        // Create the HMAC of the password using the nonce received as challenge
        let key = state.nonce.to_le_bytes();
        let hmac = hmac::sign(&key, pwd).map_err(|_| zerror!("{S} Encoding error."))?;
        if hmac != open_syn.hmac {
            bail!("{S} Invalid password.");
        }
        let username = open_syn.user.to_owned();
        Ok(username)
    }

    /// Acknowledges a successfully verified username/password exchange.
    ///
    /// # Arguments
///
    /// * `_input` - Acceptor state used by the handshake implementation.
    ///
    /// # Returns
///
    /// The unit acknowledgement extension.
    async fn send_open_ack(
        self,
        _input: Self::SendOpenAckIn,
    ) -> Result<Self::SendOpenAckOut, Self::Error> {
        Ok(Some(ZExtUnit::new()))
    }
}

/// Checks that configured credentials and dictionary files are handled correctly.
#[cfg(test)]
mod tests {

    /// Accepts valid configuration and rejects malformed dictionary entries.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn authenticator_usrpwd_config() {
        async fn inner() {
            use std::fs::File;
            use std::io::Write;

            use zenoh_config::UsrPwdConf;

            use super::AuthUsrPwd;

            // [CONFIG]
            let f1 = "zenoh-test-auth-usrpwd.txt";

            let mut config = UsrPwdConf::default();
            config.set_user(Some("usr1".to_owned())).unwrap();
            config.set_password(Some("pwd1".to_owned())).unwrap();
            config.set_dictionary_file(Some(f1.to_owned())).unwrap();

            macro_rules! zconfig {
                () => {
                    File::options()
                        .create(true)
                        .write(true)
                        .truncate(true)
                        .open(f1)
                        .unwrap()
                };
            }
            // Valid config
            let mut c = zconfig!();
            writeln!(c, "usr1:pwd1").unwrap();
            drop(c);
            assert!(AuthUsrPwd::from_config(&config).await.unwrap().is_some());
            // Invalid config
            let mut c = zconfig!();
            writeln!(c, "usr1").unwrap();
            drop(c);
            assert!(AuthUsrPwd::from_config(&config).await.is_err());
            // Empty password
            let mut c = zconfig!();
            writeln!(c, "usr1:").unwrap();
            drop(c);
            assert!(AuthUsrPwd::from_config(&config).await.is_err());
            // Empty user
            let mut c = zconfig!();
            writeln!(c, ":pwd1").unwrap();
            drop(c);
            assert!(AuthUsrPwd::from_config(&config).await.is_err());
            // Empty user and password
            let mut c = zconfig!();
            writeln!(c, ":").unwrap();
            drop(c);
            assert!(AuthUsrPwd::from_config(&config).await.is_err());

            let _ = std::fs::remove_file(f1);
        }

        inner().await;
    }
}
