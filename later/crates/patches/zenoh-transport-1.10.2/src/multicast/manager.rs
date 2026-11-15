//! Owns multicast transport configuration, peer state, and link lifecycle.
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

use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::Mutex;
#[cfg(feature = "transport_compression")]
use zenoh_config::CompressionMulticastConf;
use zenoh_config::Config;
use zenoh_config::LinkTxConf;
use zenoh_core::zasynclock;
use zenoh_link::*;
use zenoh_protocol::core::ZenohIdProto;
use zenoh_protocol::core::parameters;
use zenoh_protocol::transport::close;
use zenoh_result::ZResult;
use zenoh_result::bail;
use zenoh_result::zerror;

use crate::TransportManager;
use crate::multicast::TransportMulticast;
use crate::multicast::transport::TransportMulticastInner;
/// Configuration applied when creating a multicast transport manager.

#[derive(Debug)]
pub struct TransportManagerConfigMulticast {
    /// Lease duration advertised by the peer.
    pub lease: Duration,
    /// Maximum missed keep-alive intervals before closing.
    pub keep_alive: usize,
    /// Interval between background task joins.
    pub join_interval: Duration,
    /// Maximum concurrent sessions.
    pub max_sessions: usize,
    /// Whether the peer negotiated QoS support.
    pub is_qos: bool,
    #[cfg(feature = "transport_compression")]
    /// Whether compression is enabled for this batch.
    pub is_compression: bool,
}
/// Builder for a multicast transport manager.

pub struct TransportManagerBuilderMulticast {
    /// Lease duration advertised by the peer.
    lease: Duration,
    /// Maximum missed keep-alive intervals before closing.
    keep_alive: usize,
    /// Interval between background task joins.
    join_interval: Duration,
    /// Maximum concurrent sessions.
    max_sessions: usize,
    /// Flag indicating whether the transport manager builder multicast satisfies the is qos
    /// condition.
    is_qos: bool,
    #[cfg(feature = "transport_compression")]
    /// Flag indicating whether the transport manager builder multicast satisfies the is
    /// compression condition.
    is_compression: bool,
}

impl fmt::Debug for TransportManagerBuilderMulticast {
    /// Formats the value for display.
    /// # Arguments
    /// * `f` - Mutable formatter whose state is updated by this operation.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct("TransportManagerBuilderMulticast");
        debug
            .field("lease", &self.lease)
            .field("keep_alive", &self.keep_alive)
            .field("join_interval", &self.join_interval)
            .field("max_sessions", &self.max_sessions)
            .field("is_qos", &self.is_qos);
        #[cfg(feature = "transport_compression")]
        debug.field("is_compression", &self.is_compression);
        debug.finish()
    }
}
/// Runtime state owned by a multicast transport manager.

pub struct TransportManagerStateMulticast {
    // Established listeners
    /// Collection of link managers values consumed by transport manager state multicast
    /// operations.
    pub(crate) link_managers: Arc<Mutex<HashMap<LinkKind, LinkManagerMulticast>>>,
    // Established transports
    /// Collection of transports values consumed by transport manager state multicast operations.
    pub(crate) transports: Arc<Mutex<HashMap<Locator, Arc<TransportMulticastInner>>>>,
}

impl fmt::Debug for TransportManagerStateMulticast {
    /// Formats the value for display.
    /// # Arguments
    /// * `f` - Mutable formatter whose state is updated by this operation.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TransportManagerStateMulticast")
            .field("link_managers", &"..")
            .field("transports", &"..")
            .finish()
    }
}

/// State required by `TransportManagerParamsMulticast` during transport operations.
pub struct TransportManagerParamsMulticast {
    /// Settings that control how this transport manager params multicast is constructed or
    /// processed.
    pub config: TransportManagerConfigMulticast,
    /// Current lifecycle value used to decide which transport manager params multicast operations
    /// are valid.
    pub state: TransportManagerStateMulticast,
}

impl fmt::Debug for TransportManagerParamsMulticast {
    /// Formats the value for display.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TransportManagerParamsMulticast")
            .field("config", &self.config)
            .field("state", &self.state)
            .finish()
    }
}

impl TransportManagerBuilderMulticast {
    /// # Arguments
    /// * `lease` - Duration controlling the operation.
    pub fn lease(mut self, lease: Duration) -> Self {
        self.lease = lease;
        self
    }

    pub fn keep_alive(mut self, keep_alive: usize) -> Self {
        self.keep_alive = keep_alive;
        self
    }

    /// # Arguments
    /// * `join_interval` - Duration controlling the operation.

    pub fn join_interval(mut self, join_interval: Duration) -> Self {
        self.join_interval = join_interval;
        self
    }

    pub fn max_sessions(mut self, max_sessions: usize) -> Self {
        self.max_sessions = max_sessions;
        self
    }

    pub fn qos(mut self, is_qos: bool) -> Self {
        self.is_qos = is_qos;
        self
    }

    #[cfg(feature = "transport_compression")]
    pub fn compression(mut self, is_compression: bool) -> Self {
        self.is_compression = is_compression;
        self
    }

    /// Converts between the transport manager builder multicast representation and config while
    /// preserving the invariants required by callers.
    ///
    /// # Arguments
    /// * `config` - Settings that control the behavior and limits of this transport manager builder
    ///   multicast workflow.

    /// # Panics
    /// Panics if an internal `Result` or `Option` has no usable value.
    pub fn from_config(mut self, config: &Config) -> ZResult<TransportManagerBuilderMulticast> {
        self = self.lease(Duration::from_millis(
            *config.transport().link().tx().lease(),
        ));
        self = self.keep_alive(*config.transport().link().tx().keep_alive());
        self = self.join_interval(Duration::from_millis(
            config.transport().multicast().join_interval().unwrap(),
        ));
        self = self.max_sessions(config.transport().multicast().max_sessions().unwrap());
        self = self.qos(*config.transport().multicast().qos().enabled());
        #[cfg(feature = "transport_compression")]
        {
            self = self.compression(*config.transport().multicast().compression().enabled());
        }

        Ok(self)
    }

    /// Builds the current value.
    ///
    /// # Errors
    /// Returns an error if the requested resource or connection cannot be created.

    pub fn build(self) -> ZResult<TransportManagerParamsMulticast> {
        let config = TransportManagerConfigMulticast {
            lease: self.lease,
            keep_alive: self.keep_alive,
            join_interval: self.join_interval,
            max_sessions: self.max_sessions,
            is_qos: self.is_qos,
            #[cfg(feature = "transport_compression")]
            is_compression: self.is_compression,
        };

        let state = TransportManagerStateMulticast {
            link_managers: Arc::new(Mutex::new(HashMap::new())),
            transports: Arc::new(Mutex::new(HashMap::new())),
        };

        let params = TransportManagerParamsMulticast { config, state };

        Ok(params)
    }
}

impl Default for TransportManagerBuilderMulticast {
    /// Constructs a usable initial transport manager builder multicast value with the defaults
    /// required by the manager component.
    ///
    /// # Panics
    /// Panics if an internal `Result` or `Option` has no usable value.
    fn default() -> TransportManagerBuilderMulticast {
        let link_tx = LinkTxConf::default();
        #[cfg(feature = "transport_compression")]
        let compression = CompressionMulticastConf::default();

        let tmb = TransportManagerBuilderMulticast {
            lease: Duration::from_millis(*link_tx.lease()),
            keep_alive: *link_tx.keep_alive(),
            join_interval: Duration::from_millis(0),
            max_sessions: 0,
            is_qos: false,
            #[cfg(feature = "transport_compression")]
            is_compression: *compression.enabled(),
        };
        tmb.from_config(&Config::default()).unwrap()
    }
}

impl TransportManager {
    pub fn config_multicast() -> TransportManagerBuilderMulticast {
        TransportManagerBuilderMulticast::default()
    }

    /// Closes multicast.

    pub async fn close_multicast(&self) {
        tracing::trace!("TransportManagerMulticast::clear()");

        zasynclock!(self.state.multicast.link_managers).clear();
        let mut guard = zasynclock!(self.state.multicast.transports);
        let transports = std::mem::take(&mut *guard);
        // drop guard: mutex is acquired down the tm.close callstack
        drop(guard);
        for (_, tm) in transports {
            let _ = tm.close(close::reason::GENERIC).await;
        }
    }

    /// **********************************
    // LINK MANAGER
    /// Creates link manager multicast.
    /// **********************************
    async fn new_link_manager_multicast(
        &self,
        endpoint: &EndPoint,
    ) -> ZResult<LinkManagerMulticast> {
        let link_kind = LinkKind::try_from(endpoint)?;
        if !self.config.supported_links.contains(&link_kind) {
            bail!(
                "Unsupported link: {:?}. Supported links are: {:?}",
                link_kind,
                self.config.supported_links
            );
        }

        let mut w_guard = zasynclock!(self.state.multicast.link_managers);
        match w_guard.get(&link_kind) {
            Some(lm) => Ok(lm.clone()),
            None => {
                let lm = LinkManagerBuilderMulticast::make(link_kind)?;
                w_guard.insert(link_kind, lm.clone());
                Ok(lm)
            }
        }
    }

    /// # Errors
    /// An error is returned if a fallible operation fails.
    async fn del_link_manager_multicast(&self, link_kind: LinkKind) -> ZResult<()> {
        match zasynclock!(self.state.multicast.link_managers).remove(&link_kind) {
            Some(_) => Ok(()),
            None => bail!(
                "Can not delete the link manager for link ({link_kind:?}) because it has not been found."
            ),
        }
    }

    /// **********************************
    // TRANSPORT
    /// Opens transport multicast.
    ///
    /// # Arguments
    /// **********************************
    ///
    /// # Errors
    /// Returns an error if the requested resource or connection cannot be created.
    pub async fn open_transport_multicast(
        &self,
        mut endpoint: EndPoint,
    ) -> ZResult<TransportMulticast> {
        let p = endpoint.protocol();
        let link_kind = LinkKind::try_from(&endpoint)?;
        if !self.config.supported_links.contains(&link_kind) {
            bail!(
                "Unsupported protocol: {}. Supported protocols are: {:?}",
                p,
                self.config.supported_links
            );
        }
        if !self
            .locator_inspector
            .is_multicast(&endpoint.to_locator())
            .await?
        {
            bail!(
                "Can not open a multicast transport with a unicast endpoint: {}.",
                endpoint
            )
        }

        // Automatically create a new link manager for the protocol if it does not exist
        let manager = self.new_link_manager_multicast(&endpoint).await?;
        // Fill and merge the endpoint configuration
        if let Some(config) = self
            .config
            .link_configs
            .get(&LinkKind::try_from(&endpoint)?)
        {
            let mut config = parameters::Parameters::from(config.as_str());
            // Overwrite config with current endpoint parameters
            config.extend_from_iter(endpoint.config().iter());
            endpoint = EndPoint::new(
                endpoint.protocol(),
                endpoint.address(),
                endpoint.metadata(),
                config.as_str(),
            )?;
        }

        // Open the link
        let link = manager.new_link(&endpoint).await?;
        super::establishment::open_link(self, link).await
    }

    pub async fn get_transport_multicast(&self, zid: &ZenohIdProto) -> Option<TransportMulticast> {
        for t in zasynclock!(self.state.multicast.transports).values() {
            if t.get_peers().iter().any(|p| p.zid == *zid) {
                return Some(t.into());
            }
        }
        None
    }

    pub async fn get_transports_multicast(&self) -> Vec<TransportMulticast> {
        zasynclock!(self.state.multicast.transports)
            .values()
            .map(|t| t.into())
            .collect()
    }

    /// Retrieves the transports multicast blocking for the current transport manager from its

    pub fn get_transports_multicast_blocking(&self) -> Vec<TransportMulticast> {
        self.state
            .multicast
            .transports
            .blocking_lock()
            .values()
            .map(|t| t.into())
            .collect()
    }

    /// # Errors
    /// An error is returned if try_from fails.
    pub(super) async fn del_transport_multicast(&self, locator: &Locator) -> ZResult<()> {
        let mut guard = zasynclock!(self.state.multicast.transports);
        let res = guard.remove(locator);
        let link_kind = LinkKind::try_from(locator)?;

        if !guard
            .iter()
            .any(|(l, _)| LinkKind::try_from(l).is_ok_and(|k| k == link_kind))
        {
            let _ = self.del_link_manager_multicast(link_kind).await;
        }

        res.map(|_| ()).ok_or_else(|| {
            let e = zerror!("Can not delete the transport for locator: {}", locator);
            tracing::trace!("{}", e);
            e.into()
        })
    }

    /// **********************************
    // LISTENER
    /// # Arguments
    /// **********************************
    pub async fn add_listener_multicast(&self, endpoint: EndPoint) -> ZResult<Locator> {
        let locator = endpoint.to_locator().to_owned();
        self.open_transport_multicast(endpoint).await?;
        Ok(locator)
    }

    /// # Errors
    /// An error is returned if try_from fails.
    pub async fn del_listener_multicast(&self, endpoint: &EndPoint) -> ZResult<()> {
        let locator = endpoint.to_locator();

        let mut guard = zasynclock!(self.state.multicast.transports);
        let res = guard.remove(&locator);
        let link_kind = LinkKind::try_from(&locator)?;

        if !guard
            .iter()
            .any(|(l, _)| LinkKind::try_from(l).is_ok_and(|k| k == link_kind))
        {
            let _ = self.del_link_manager_multicast(link_kind).await;
        }

        res.map(|_| ()).ok_or_else(|| {
            let e = zerror!("Can not delete the transport for locator: {}", locator);
            tracing::trace!("{}", e);
            e.into()
        })
    }

    pub async fn get_listeners_multicast(&self) -> Vec<EndPoint> {
        zasynclock!(self.state.multicast.transports)
            .values()
            .map(|t| t.locator.clone().into())
            .collect()
    }

    pub async fn get_locators_multicast(&self) -> Vec<Locator> {
        zasynclock!(self.state.multicast.transports)
            .values()
            .map(|t| t.locator.clone())
            .collect()
    }
}
