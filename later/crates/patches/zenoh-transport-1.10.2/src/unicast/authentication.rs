//! Authenticates remote peers during unicast transport establishment.
//!
//! TODO Reviewed ??-??-2026.

// Copyright (c) 2024 ZettaScale Technology
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

use zenoh_link::LinkAuthId;
use zenoh_protocol::core::ZenohIdProto;

#[cfg(feature = "auth_usrpwd")]
use super::establishment::ext::auth::UsrPwdId;
/// Authentication identity associated with a unicast transport.

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransportAuthId {
    /// Optional username used by transport auth id; `None` means the value is not configured or
    /// unavailable.
    username: Option<String>,
    /// Zenoh identifier of this peer.
    zid: ZenohIdProto,
    /// Collection of link auth ids values consumed by transport auth id operations.
    link_auth_ids: Vec<LinkAuthId>,
}

impl TransportAuthId {
    /// Constructs a usable initial transport auth id value with the defaults required by the
    /// authentication component.
    pub(crate) fn new(zid: ZenohIdProto) -> Self {
        Self {
            username: None,
            zid,
            link_auth_ids: vec![],
        }
    }

    /// Changes the username held by the transport auth id; subsequent operations use the updated
    /// value.
    ///
    /// # Arguments
    /// * `user_pwd_id` - Identifier of the user pwd.
    #[cfg(feature = "auth_usrpwd")]
    pub(crate) fn set_username(&mut self, user_pwd_id: &UsrPwdId) {
        self.username = if let Some(username) = &user_pwd_id.0 {
            // Convert username from Vec<u8> to String
            match std::str::from_utf8(username) {
                Ok(name) => Some(name.to_owned()),
                Err(e) => {
                    tracing::error!("Error in extracting username {}", e);
                    None
                }
            }
        } else {
            None
        }
    }

    /// Adds link auth id.
    ///
    /// # Arguments
    /// * `link_auth_id` - Identifier of the link auth.

    pub(crate) fn push_link_auth_id(&mut self, link_auth_id: LinkAuthId) {
        self.link_auth_ids.push(link_auth_id);
    }

    pub fn username(&self) -> Option<&String> {
        self.username.as_ref()
    }

    pub fn link_auth_ids(&self) -> &Vec<LinkAuthId> {
        &self.link_auth_ids
    }

    pub fn zid(&self) -> &ZenohIdProto {
        &self.zid
    }
}
