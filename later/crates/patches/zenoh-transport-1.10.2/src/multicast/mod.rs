//! Organizes the multicast subsystem and declares its child modules.
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
/// Establishes multicast transports and registers newly discovered peers.
pub(crate) mod establishment;
/// Represents a multicast link and exposes its transport read and write operations.
pub(crate) mod link;
/// Owns multicast transport configuration, peer state, and link lifecycle.
pub(crate) mod manager;
/// Receives and dispatches messages arriving on a multicast transport.
pub(crate) mod rx;
/// Tracks multicast transport state and coordinates its links, peers, and priority queues.
pub(crate) mod transport;
/// Schedules and sends messages over multicast transport links.
pub(crate) mod tx;

use std::fmt::Write;
use std::fmt::{self};
use std::sync::Arc;
use std::sync::Weak;

pub use manager::TransportManagerBuilderMulticast;
pub use manager::TransportManagerConfigMulticast;
pub use manager::TransportManagerParamsMulticast;
use transport::TransportMulticastInner;
use zenoh_core::zcondfeat;
use zenoh_core::zread;
use zenoh_link::Link;
use zenoh_protocol::core::Bits;
use zenoh_protocol::network::NetworkMessageMut;
use zenoh_protocol::transport::PrioritySn;
use zenoh_protocol::transport::close;
use zenoh_result::ZResult;
use zenoh_result::zerror;

use super::common;
use crate::TransportMulticastEventHandler;
use crate::TransportPeer;
use crate::multicast::link::TransportLinkMulticast;

/// **********************************
// TRANSPORT MULTICAST
/// **********************************
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct TransportConfigMulticast {
    /// Number of bits used for sequence numbers.
    pub(crate) sn_resolution: Bits,
    /// Initial sns value.
    pub(crate) initial_sns: Box<[PrioritySn]>,
    /// Transport link associated with this state.
    pub(crate) link: TransportLinkMulticast,
}
/// Multicast transport state shared by its connected peers and links.

#[derive(Clone)]
pub struct TransportMulticast(Weak<TransportMulticastInner>);

impl TransportMulticast {
    #[inline(always)]
    fn get_transport(&self) -> ZResult<Arc<TransportMulticastInner>> {
        self.0
            .upgrade()
            .ok_or_else(|| zerror!("Transport multicast closed").into())
    }

    /// # Errors
    /// An error is returned if get_sn_resolution fails.
    /// Returns an error if the requested data cannot be retrieved.
    #[inline(always)]
    pub fn get_sn_resolution(&self) -> ZResult<Bits> {
        let transport = self.get_transport()?;
        Ok(transport.get_sn_resolution())
    }

    /// Reports whether the transport multicast meets the shm condition so callers can choose the
    /// correct branch.

    /// # Errors
    /// An error is returned if is_shm fails.
    #[inline(always)]
    pub fn is_shm(&self) -> ZResult<bool> {
        let transport = self.get_transport()?;
        Ok(transport.is_shm())
    }

    /// Reports whether the transport multicast meets the qos condition so callers can choose the
    /// correct branch.

    /// # Errors
    /// An error is returned if is_qos fails.
    pub fn is_qos(&self) -> ZResult<bool> {
        let transport = self.get_transport()?;
        Ok(transport.is_qos())
    }

    /// # Errors
    /// An error is returned if get_callback fails.
    /// Returns an error if the requested data cannot be retrieved.
    pub fn get_callback(&self) -> ZResult<Option<Arc<dyn TransportMulticastEventHandler>>> {
        let transport = self.get_transport()?;
        Ok(transport.get_callback())
    }

    /// # Errors
    /// An error is returned if get_link fails.
    /// Returns an error if the requested data cannot be retrieved.
    pub fn get_link(&self) -> ZResult<Link> {
        let transport = self.get_transport()?;
        Ok(Link::new_multicast(&transport.get_link().link))
    }

    /// # Errors
    /// An error is returned if get_peers fails.
    /// Returns an error if the requested data cannot be retrieved.
    pub fn get_peers(&self) -> ZResult<Vec<TransportPeer>> {
        let transport = self.get_transport()?;
        Ok(transport.get_peers())
    }

    #[inline(always)]
    pub async fn close(&self) -> ZResult<()> {
        // Return Ok if the transport has already been closed
        match self.get_transport() {
            Ok(transport) => transport.close(close::reason::GENERIC).await,
            Err(_) => Ok(()),
        }
    }

    /// Returns if the message has successfully been sent.
    ///
    /// # Arguments
    /// * `message` - Message to send or handle.
    ///
    /// # Errors
    /// Returns an error if the operation cannot be completed.
    #[inline(always)]
    pub fn schedule(&self, message: NetworkMessageMut) -> ZResult<bool> {
        let transport = self.get_transport()?;
        transport.schedule(message)
    }

    /// # Errors
    /// An error is returned if get_stats fails.
    /// Returns an error if the requested data cannot be retrieved.
    pub fn get_stats(&self) -> ZResult<zenoh_stats::TransportStats> {
        Ok(self.get_transport()?.stats.clone())
    }
}

impl From<&Arc<TransportMulticastInner>> for TransportMulticast {
    fn from(s: &Arc<TransportMulticastInner>) -> TransportMulticast {
        TransportMulticast(Arc::downgrade(s))
    }
}

impl Eq for TransportMulticast {}

impl PartialEq for TransportMulticast {
    fn eq(&self, other: &Self) -> bool {
        Weak::ptr_eq(&self.0, &other.0)
    }
}

impl fmt::Debug for TransportMulticast {
    /// Formats the value for display.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.get_transport() {
            Ok(transport) => {
                let is_shm = zcondfeat!("shared-memory", transport.is_shm(), false);
                let peers: String =
                    zread!(transport.peers)
                        .iter()
                        .fold(String::new(), |mut output, (l, p)| {
                            let _ = write!(
                                output,
                                "(locator: {}, zid: {}, whatami: {})",
                                l, p.zid, p.whatami
                            );
                            output
                        });

                f.debug_struct("Transport Multicast")
                    .field("sn_resolution", &transport.get_sn_resolution())
                    .field("is_qos", &transport.is_qos())
                    .field("is_shm", &is_shm)
                    .field("peers", &peers)
                    .finish()
            }
            Err(e) => {
                write!(f, "{e}")
            }
        }
    }
}
