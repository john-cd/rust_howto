//! ⚠️ WARNING ⚠️
//! Transport types and event callbacks shared by Zenoh unicast and multicast links.
//! This crate is intended for Zenoh's internal use.
//! [Click here for Zenoh's documentation](https://docs.rs/zenoh/latest/zenoh)
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

//! ⚠️ WARNING ⚠️
//!
//! This crate is intended for Zenoh's internal use.
//!
//! [Click here for Zenoh's documentation](https://docs.rs/zenoh/latest/zenoh)
/// Provides common behavior for the src component.
pub mod common;
/// Provides manager behavior for the src component.
pub mod manager;
/// Provides multicast behavior for the src component.
pub mod multicast;
/// Provides unicast behavior for the src component.
pub mod unicast;

#[cfg(all(
    feature = "uring",
    target_os = "linux",
    any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "riscv64",
        target_arch = "loongarch64",
        target_arch = "powerpc64"
    )
))]
/// Provides uring behavior for the src component.
mod uring;

use std::any::Any;
use std::sync::Arc;

pub use manager::*;
use serde::Serialize;
use zenoh_link::Link;
use zenoh_protocol::core::RegionName;
use zenoh_protocol::core::WhatAmI;
use zenoh_protocol::core::ZenohIdProto;
use zenoh_protocol::network::NetworkMessageMut;
use zenoh_result::ZResult;

use crate::multicast::TransportMulticast;
use crate::unicast::TransportUnicast;

/// **********************************
// TRANSPORT

/// **********************************
pub trait TransportEventHandler: Send + Sync {
    /// Handles the new unicast behavior for this component.
    fn new_unicast(
        &self,
        peer: TransportPeer,
        transport: TransportUnicast,
    ) -> ZResult<Arc<dyn TransportPeerEventHandler>>;

    /// Handles the new multicast behavior for this component.
    fn new_multicast(
        &self,
        _transport: TransportMulticast,
    ) -> ZResult<Arc<dyn TransportMulticastEventHandler>>;
}

/// Data for dummy transport event handler operations, including peer and transport.
#[derive(Debug, Default)]
pub struct DummyTransportEventHandler;

impl TransportEventHandler for DummyTransportEventHandler {
    /// Handles the new unicast behavior for this component.
    fn new_unicast(
        &self,
        _peer: TransportPeer,
        _transport: TransportUnicast,
    ) -> ZResult<Arc<dyn TransportPeerEventHandler>> {
        Ok(Arc::new(DummyTransportPeerEventHandler))
    }

    /// Handles the new multicast behavior for this component.
    fn new_multicast(
        &self,
        _transport: TransportMulticast,
    ) -> ZResult<Arc<dyn TransportMulticastEventHandler>> {
        Ok(Arc::new(DummyTransportMulticastEventHandler))
    }
}

/// **********************************
// MULTICAST

/// **********************************
pub trait TransportMulticastEventHandler: Send + Sync {
    /// Handles the new peer behavior for this component.
    fn new_peer(&self, peer: TransportPeer) -> ZResult<Arc<dyn TransportPeerEventHandler>>;
    /// Reports whether the transport channel has been closed.
    fn closed(&self);
    /// Views or converts this value as a any.
    fn as_any(&self) -> &dyn Any;
}

// Define an empty TransportCallback for the listener transport
/// Data carried by dummy transport multicast event handler operations.
#[derive(Debug, Default)]
pub struct DummyTransportMulticastEventHandler;

impl TransportMulticastEventHandler for DummyTransportMulticastEventHandler {
    /// Handles the new peer behavior for this component.
    fn new_peer(&self, _peer: TransportPeer) -> ZResult<Arc<dyn TransportPeerEventHandler>> {
        Ok(Arc::new(DummyTransportPeerEventHandler))
    }

    /// Reports whether the transport channel has been closed.
    fn closed(&self) {}

    /// Views or converts this value as a any.
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// **********************************
// CALLBACK

/// **********************************
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
#[serde(rename = "Transport")]
pub struct TransportPeer {
    /// Zenoh identifier of the remote peer.
    pub zid: ZenohIdProto,
    /// Role advertised by the remote Zenoh peer.
    pub whatami: WhatAmI,
    /// Whether quality-of-service support was negotiated.
    pub is_qos: bool,
    /// Active links that carry transport traffic.
    #[serde(skip)]
    pub links: Vec<Link>,
    /// Whether shared-memory transport was negotiated.
    #[cfg(feature = "shared-memory")]
    pub is_shm: bool,
    /// Shared-memory region name advertised during establishment.
    pub region_name: Option<RegionName>,
}

/// Defines the operations required for transport peer event handler behavior.
pub trait TransportPeerEventHandler: Send + Sync {
    /// Handles the supplied message for this component.
    fn handle_message(&self, msg: NetworkMessageMut) -> ZResult<()>;
    /// Handles the new link behavior for this component.
    fn new_link(&self, src: Link);
    /// Removes the link after its transport state has been shut down.
    fn del_link(&self, link: Link);
    /// Reports whether the transport channel has been closed.
    fn closed(&self);
    /// Views or converts this value as a any.
    fn as_any(&self) -> &dyn Any;
}

// Define an empty TransportCallback for the listener transport
/// Data carried by dummy transport peer event handler operations.
#[derive(Debug, Default)]
pub struct DummyTransportPeerEventHandler;

impl TransportPeerEventHandler for DummyTransportPeerEventHandler {
    /// Handles the supplied message for this component.
    fn handle_message(&self, _message: NetworkMessageMut) -> ZResult<()> {
        Ok(())
    }

    /// Handles the new link behavior for this component.
    fn new_link(&self, _link: Link) {}

    /// Removes the link after its transport state has been shut down.
    fn del_link(&self, _link: Link) {}

    /// Reports whether the transport channel has been closed.
    fn closed(&self) {}

    /// Views or converts this value as a any.
    fn as_any(&self) -> &dyn Any {
        self
    }
}
