//! Organizes the unicast subsystem and declares its child modules.
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
/// Provides authentication behavior for the unicast component.
pub mod authentication;
/// Provides establishment behavior for the unicast component.
pub mod establishment;
/// Provides link behavior for the unicast component.
pub(crate) mod link;
/// Provides lowlatency behavior for the unicast component.
pub(crate) mod lowlatency;
/// Provides manager behavior for the unicast component.
pub(crate) mod manager;
/// Provides test helpers behavior for the unicast component.
#[cfg(feature = "test")]
pub mod test_helpers;
/// Provides transport unicast inner behavior for the unicast component.
pub(crate) mod transport_unicast_inner;
/// Provides universal behavior for the unicast component.
pub(crate) mod universal;

use std::fmt;
use std::sync::Arc;
use std::sync::Weak;

#[cfg(feature = "transport_multilink")]
use establishment::ext::auth::ZPublicKey;
pub use manager::*;
use zenoh_core::zcondfeat;
use zenoh_link::Link;
use zenoh_protocol::core::Bits;
use zenoh_protocol::core::Bound;
use zenoh_protocol::core::RegionName;
use zenoh_protocol::core::WhatAmI;
use zenoh_protocol::core::ZenohIdProto;
use zenoh_protocol::network::NetworkMessageMut;
use zenoh_protocol::transport::TransportSn;
use zenoh_protocol::transport::close;
use zenoh_protocol::transport::init::ext::PatchType;
use zenoh_result::ZResult;
use zenoh_result::zerror;

use self::transport_unicast_inner::TransportUnicastTrait;
use super::TransportPeer;
use super::TransportPeerEventHandler;
#[cfg(feature = "shared-memory")]
use crate::common::shm::interop::TransportShmConfig;
use crate::unicast::authentication::TransportAuthId;
#[cfg(feature = "auth_usrpwd")]
use crate::unicast::establishment::ext::auth::UsrPwdId;

/// **********************************
// TRANSPORT UNICAST

/// **********************************
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct TransportConfigUnicast {
    /// Zenoh identifier of the remote peer.
    pub(crate) zid: ZenohIdProto,
    /// Role advertised by the remote Zenoh peer.
    pub(crate) whatami: WhatAmI,
    /// Shared-memory region name advertised during establishment.
    pub(crate) region_name: Option<RegionName>,
    /// Upper bound used to limit the allocation or search.
    pub(crate) bound: Option<Bound>,
    /// sn resolution value tracked by this type.
    pub(crate) sn_resolution: Bits,
    /// Initial transmit sequence number negotiated for this channel.
    pub(crate) tx_initial_sn: TransportSn,
    /// Whether quality-of-service support was negotiated.
    pub(crate) is_qos: bool,
    /// Negotiated state for the multilink transport extension.
    #[cfg(feature = "transport_multilink")]
    pub(crate) multilink: Option<ZPublicKey>,
    /// Shared-memory extension state negotiated for the session.
    #[cfg(feature = "shared-memory")]
    pub(crate) shm: Option<TransportShmConfig>,
    /// Whether lowlatency is enabled for this value.
    pub(crate) is_lowlatency: bool,
    /// Identifier of the authenticated peer.
    #[cfg(feature = "auth_usrpwd")]
    pub(crate) auth_id: UsrPwdId,
    /// Patch type negotiated during the transport open handshake.
    pub(crate) patch: PatchType,
}

/// [`TransportUnicast`] is the transport handler returned
/// when opening a new unicast transport
#[derive(Clone)]
pub struct TransportUnicast(
    /// Weak reference to unicast internals that avoids a reference cycle.
    Weak<dyn TransportUnicastTrait>,
);

impl TransportUnicast {
    /// Returns the inner stored by or available from this value.
    #[inline(always)]
    pub(super) fn get_inner(&self) -> ZResult<Arc<dyn TransportUnicastTrait>> {
        self.0
            .upgrade()
            .ok_or_else(|| zerror!("Transport unicast closed").into())
    }

    /// Returns the zid stored by or available from this value.
    #[inline(always)]
    pub fn get_zid(&self) -> ZResult<ZenohIdProto> {
        let transport = self.get_inner()?;
        Ok(transport.get_zid())
    }

    /// Returns the whatami stored by or available from this value.
    #[inline(always)]
    pub fn get_whatami(&self) -> ZResult<WhatAmI> {
        let transport = self.get_inner()?;
        Ok(transport.get_whatami())
    }

    /// Returns the bound stored by or available from this value.
    #[inline(always)]
    pub fn get_bound(&self) -> ZResult<Option<Bound>> {
        let transport = self.get_inner()?;
        Ok(transport.get_bound())
    }

    /// Reports whether this value is shm.
    #[cfg(feature = "shared-memory")]
    #[inline(always)]
    pub fn is_shm(&self) -> ZResult<bool> {
        let transport = self.get_inner()?;
        Ok(transport.is_shm())
    }

    /// Returns the callback stored by or available from this value.
    #[inline(always)]
    pub fn get_callback(&self) -> ZResult<Option<Arc<dyn TransportPeerEventHandler>>> {
        let transport = self.get_inner()?;
        Ok(transport.get_callback())
    }

    /// Returns the peer stored by or available from this value.
    pub fn get_peer(&self) -> ZResult<TransportPeer> {
        let transport = self.get_inner()?;
        let tp = TransportPeer {
            zid: transport.get_zid(),
            whatami: transport.get_whatami(),
            links: transport.get_links(),
            is_qos: transport.is_qos(),
            #[cfg(feature = "shared-memory")]
            is_shm: transport.is_shm(),
            region_name: transport.region_name(),
        };
        Ok(tp)
    }

    /// Returns the links stored by or available from this value.
    #[inline(always)]
    pub fn get_links(&self) -> ZResult<Vec<Link>> {
        let transport = self.get_inner()?;
        Ok(transport.get_links())
    }

    /// Returns the auth ids stored by or available from this value.
    pub fn get_auth_ids(&self) -> ZResult<TransportAuthId> {
        let transport = self.get_inner()?;
        Ok(transport.get_auth_ids())
    }

    /// Schedules the event at its calculated execution time.
    #[inline(always)]
    pub fn schedule(&self, message: NetworkMessageMut) -> ZResult<bool> {
        let transport = self.get_inner()?;
        transport.schedule(message)
    }

    /// Closes the transport resource and releases its associated state.
    #[inline(always)]
    pub async fn close(&self) -> ZResult<()> {
        // Return Ok if the transport has already been closed
        match self.get_inner() {
            Ok(transport) => transport.close(close::reason::GENERIC).await,
            Err(_) => Ok(()),
        }
    }

    /// Close a specific link within this transport.
    ///
    /// If the transport has multiple links (multilink), only the specified
    /// link is closed and the transport remains alive. If this is the last
    /// link, the entire transport is closed.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example(transport: zenoh_transport::unicast::TransportUnicast) {
    /// // Close a specific link while keeping the transport alive
    /// let links = transport.get_links().unwrap();
    /// if links.len() > 1 {
    ///     transport.close_link(links[0].clone()).await.unwrap();
    /// }
    /// # }
    /// ```
    #[inline(always)]
    pub async fn close_link(&self, link: Link) -> ZResult<()> {
        let transport = self.get_inner()?;
        transport.close_link(link).await
    }

    /// Returns the transport stats, or an error if the transport is closed.
    ///
    /// Warning: returning an error prevents interceptors to initialize;
    /// if the error changes in the future, updating interceptors may be necessary.
    #[cfg(feature = "stats")]
    pub fn get_stats(&self) -> ZResult<zenoh_stats::TransportStats> {
        Ok(self.get_inner()?.stats())
    }
}

impl From<&Arc<dyn TransportUnicastTrait>> for TransportUnicast {
    /// Constructs this type from the supplied source value.
    fn from(link: &Arc<dyn TransportUnicastTrait>) -> TransportUnicast {
        TransportUnicast(Arc::downgrade(link))
    }
}

impl Eq for TransportUnicast {}

impl PartialEq for TransportUnicast {
    /// Compares this value with the supplied value for equality.
    fn eq(&self, other: &Self) -> bool {
        Weak::ptr_eq(&self.0, &other.0)
    }
}

impl fmt::Debug for TransportUnicast {
    /// Formats this value for human-readable output.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.get_inner() {
            Ok(transport) => {
                let is_shm = zcondfeat!("shared-memory", transport.is_shm(), false);

                transport
                    .add_debug_fields(
                        f.debug_struct("Transport Unicast")
                            .field("zid", &transport.get_zid())
                            .field("whatami", &transport.get_whatami())
                            .field("is_qos", &transport.is_qos())
                            .field("is_shm", &is_shm)
                            .field("links", &transport.get_links()),
                    )
                    .finish()
            }
            Err(e) => {
                write!(f, "{e}")
            }
        }
    }
}
