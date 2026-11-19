//! Stores shared state and coordinates the unicast transport send and receive paths.
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

//!
//

use std::fmt::DebugStruct;
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use tokio::sync::MutexGuard as AsyncMutexGuard;
use zenoh_link::Link;
use zenoh_protocol::core::Bound;
use zenoh_protocol::core::RegionName;
use zenoh_protocol::core::WhatAmI;
use zenoh_protocol::core::ZenohIdProto;
use zenoh_protocol::network::NetworkMessageMut;
use zenoh_protocol::transport::TransportSn;
use zenoh_result::ZResult;

use super::link::LinkUnicastWithOpenAck;
use super::link::MaybeOpenAck;
use crate::TransportPeerEventHandler;
use crate::unicast::TransportConfigUnicast;
use crate::unicast::link::TransportLinkUnicast;

/// Names the link error type associated with this interface.
pub(crate) type LinkError = (
    zenoh_result::Error,
    TransportLinkUnicast,
    Option<TransportLinkUnicast>,
    u8,
);
/// Alias for `(zenoh_result::Error, Arc<dyn TransportUnicastTrait>, u8)` used to represent
/// transport error.
pub(crate) type TransportError = (zenoh_result::Error, Arc<dyn TransportUnicastTrait>, u8);

/// Defines the outcomes available to the unicast workflow, including link, transport.
pub(crate) enum InitTransportError {
    /// Link variant of `InitTransportError`.
    Link(
        /// Underlying link failure reported by the transport.
        Box<LinkError>,
    ),
    /// Represents the transport case of init transport error.
    Transport(
        /// Underlying transport failure returned by the connection.
        TransportError,
    ),
}

/// Names the add link result type associated with this interface.
pub(crate) type AddLinkResult<'a> = Result<
    (
        Box<dyn FnOnce() + Send + Sync + 'a>,
        Box<dyn FnOnce() + Send + Sync + 'a>,
        MaybeOpenAck,
        AsyncMutexGuard<'a, TransportStatus>,
    ),
    LinkError,
>;
/// Alias for `Result<Arc<dyn TransportUnicastTrait>, InitTransportError>` used to represent init
/// transport result.
pub(crate) type InitTransportResult = Result<Arc<dyn TransportUnicastTrait>, InitTransportError>;

/// Defines the outcomes available to the unicast workflow, including uninitialized, alive, closed.
pub(crate) enum TransportStatus {
    /// Alive lifecycle state of the transport.
    Uninitialized,
    /// Represents the alive case of transport status.
    Alive,
    /// Represents the closed case of transport status.
    Closed,
}

/// **********************************
// UNICAST TRANSPORT TRAIT

/// **********************************
#[async_trait]
pub(crate) trait TransportUnicastTrait: Send + Sync {
    /// **********************************
    // ACCESSORS

    /// Sets callback.
    /// **********************************
    fn set_callback(&self, callback: Arc<dyn TransportPeerEventHandler>);

    /// Returns the status stored by or available from this value.
    async fn get_status(&self) -> AsyncMutexGuard<'_, TransportStatus>;
    /// Returns the zid stored by or available from this value.
    fn get_zid(&self) -> ZenohIdProto;
    /// Returns the whatami stored by or available from this value.
    fn get_whatami(&self) -> WhatAmI;
    /// Returns the callback stored by or available from this value.
    fn get_callback(&self) -> Option<Arc<dyn TransportPeerEventHandler>>;

    /// Returns the links stored by or available from this value.
    fn get_links(&self) -> Vec<Link>;
    /// Returns the auth ids stored by or available from this value.
    fn get_auth_ids(&self) -> super::authentication::TransportAuthId;

    /// Reports whether the transport unicast trait meets the shm condition so callers can choose
    /// the correct branch.
    #[cfg(feature = "shared-memory")]
    fn is_shm(&self) -> bool;

    /// Reports whether the transport unicast trait meets the qos condition so callers can choose
    /// the correct branch.
    fn is_qos(&self) -> bool;

    /// Applies the rules used by transport unicast inner to keep transport unicast trait state
    /// valid for its callers.
    fn region_name(&self) -> Option<RegionName>;
    /// Returns the bound stored by or available from this value.
    fn get_bound(&self) -> Option<Bound>;
    /// Returns the config stored by or available from this value.
    fn get_config(&self) -> &TransportConfigUnicast;

    /// Applies the rules used by transport unicast inner to keep transport unicast trait state
    /// valid for its callers.
    #[cfg(feature = "stats")]
    fn stats(&self) -> zenoh_stats::TransportStats;

    // LINK

    /// Registers a peer link and starts transport synchronization for that link.
    ///
    /// # Arguments
    ///
    /// * `link` - Accepted unicast link and its optional OpenAck response.
    /// * `other_initial_sn` - Peer sequence number used to synchronize transport state.
    /// * `other_lease` - Peer lease duration used to start the receive keep-alive timer.
    ///
    /// # Errors
    ///
    /// Returns an error if synchronization fails or the peer link cannot be registered.
    async fn add_link(
        &self,
        link: LinkUnicastWithOpenAck,
        other_initial_sn: TransportSn,
        other_lease: Duration,
    ) -> AddLinkResult;

    // TX

    /// **********************************
    /// Schedules a network message for transmission on the unicast transport.
    fn schedule(&self, msg: NetworkMessageMut) -> ZResult<bool>;

    // TERMINATION

    /// **********************************
    async fn close(&self, reason: u8) -> ZResult<()>;

    /// Close a specific link within this transport.
    ///
    /// If the transport has multiple links (multilink), only the specified
    /// link is closed and the transport remains alive. If this is the last
    /// link, the entire transport is closed.
    async fn close_link(&self, link: Link) -> ZResult<()>;

    /// Applies the rules used by transport unicast inner to keep transport unicast trait state
    /// valid for its callers.
    fn add_debug_fields<'a, 'b: 'a, 'c>(
        &self,
        s: &'c mut DebugStruct<'a, 'b>,
    ) -> &'c mut DebugStruct<'a, 'b> {
        s
    }
}
