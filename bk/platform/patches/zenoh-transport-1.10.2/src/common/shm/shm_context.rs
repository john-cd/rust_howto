//! Creates and tracks the shared-memory context attached to a transport.
//!
//! TODO Reviewed ??-??-2026.

// Copyright (c) 2025 ZettaScale Technology
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

use std::sync::Arc;

use zenoh_result::ZResult;
use zenoh_shm::api::client_storage::GLOBAL_CLIENT_STORAGE;
use zenoh_shm::reader::ShmReader;

use crate::common::shm::interop::LazyShmProvider;
use crate::common::shm::interop::MulticastTransportShmConfig;
use crate::common::shm::interop::ShmOptimizationPolicy;
use crate::common::shm::interop::TransportShmConfig;
use crate::unicast::establishment::ext::shm::auth::AuthUnicast;

/// Context passed through multicast transport shm context operations, retaining shm reader and shm
/// provider.
#[derive(Clone)]
pub(crate) struct MulticastTransportShmContext {
    /// Reader used to access the shared-memory segment.
    pub(crate) shm_reader: ShmReader,
    /// Provider used to allocate and publish shared-memory buffers.
    pub(crate) shm_provider: Option<Arc<LazyShmProvider>>,
    /// Configuration that controls shared-memory allocation.
    pub(crate) shm_config: MulticastTransportShmConfig,
    /// Policy that controls how this component handles the request.
    pub(crate) policy: ShmOptimizationPolicy,
}

impl MulticastTransportShmContext {
    /// Creates a new multicast transport shm context with the supplied configuration.
    pub(crate) fn new(
        shm_reader: ShmReader,
        shm_provider: Option<Arc<LazyShmProvider>>,
        policy: ShmOptimizationPolicy,
    ) -> Self {
        Self {
            shm_reader,
            shm_provider,
            shm_config: MulticastTransportShmConfig,
            policy,
        }
    }
}

/// Context passed through unicast transport shm context operations, retaining shm reader and shm
/// provider.
#[derive(Clone)]
pub(crate) struct UnicastTransportShmContext {
    /// Reader used to access the shared-memory segment.
    pub(crate) shm_reader: ShmReader,
    /// Provider used to allocate and publish shared-memory buffers.
    pub(crate) shm_provider: Option<Arc<LazyShmProvider>>,
    /// Configuration that controls shared-memory allocation.
    pub(crate) shm_config: TransportShmConfig,
    /// Policy that controls how this component handles the request.
    pub(crate) policy: ShmOptimizationPolicy,
}

impl UnicastTransportShmContext {
    /// Creates a new unicast transport shm context with the supplied configuration.
    pub(crate) fn new(
        shm_reader: ShmReader,
        shm_provider: Option<Arc<LazyShmProvider>>,
        shm_config: TransportShmConfig,
        policy: ShmOptimizationPolicy,
    ) -> Self {
        Self {
            shm_reader,
            shm_provider,
            shm_config,
            policy,
        }
    }
}

/// Context passed through shm context operations, retaining shm reader and shm provider.
pub struct ShmContext {
    /// Reader used to access the shared-memory segment.
    pub(crate) shm_reader: ShmReader,
    /// Provider used to allocate and publish shared-memory buffers.
    pub(crate) shm_provider: Option<Arc<LazyShmProvider>>,
    /// Policy that controls how this component handles the request.
    pub(crate) policy: ShmOptimizationPolicy,
    /// Authentication state negotiated for this session.
    pub(crate) auth: AuthUnicast,
}

impl std::fmt::Debug for ShmContext {
    /// Formats this value for human-readable output.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ShmContext")
            .field("shm_reader", &self.shm_reader)
            .field("shm_provider", &self.shm_provider)
            .field("auth", &"..")
            .finish()
    }
}

impl ShmContext {
    /// Creates a new shm context with the supplied configuration.
    pub fn new(
        cfg: zenoh_config::ShmConf,
        external_reader: Option<ShmReader>,
    ) -> ZResult<Option<Self>> {
        if !*cfg.enabled() {
            return Ok(None);
        }

        let shm_provider = if *cfg.transport_optimization.enabled() {
            Some(Arc::new(LazyShmProvider::new(
                *cfg.transport_optimization.pool_size(),
                *cfg.transport_optimization.message_size_threshold(),
            )))
        } else {
            None
        };

        let messages = cfg.transport_optimization.messages();
        let policy = ShmOptimizationPolicy {
            put: messages.contains(&zenoh_config::DataMessage::Put),
            query: messages.contains(&zenoh_config::DataMessage::Query),
            reply: messages.contains(&zenoh_config::DataMessage::Reply),
        };

        let shm_reader = external_reader
            .unwrap_or_else(|| ShmReader::new((*GLOBAL_CLIENT_STORAGE.read()).clone()));

        let auth = AuthUnicast::new(&shm_reader.supported_protocols())?;

        Ok(Some(Self {
            shm_provider,
            policy,
            shm_reader,
            auth,
        }))
    }

    /// Handles the shm provider behavior for this component.
    pub fn shm_provider(&self) -> &Option<Arc<LazyShmProvider>> {
        &self.shm_provider
    }
}
