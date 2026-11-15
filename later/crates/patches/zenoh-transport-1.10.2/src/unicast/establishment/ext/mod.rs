//! Organizes the ext subsystem and declares its child modules.
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
/// Organizes the auth subsystem and declares its child modules.
#[cfg(feature = "transport_auth")]
pub mod auth;
/// Negotiates compression settings for a unicast transport.
#[cfg(feature = "transport_compression")]
pub(crate) mod compression;
/// Negotiates the low-latency extension for a unicast transport.
pub(crate) mod lowlatency;
/// Negotiates support for multiple links on one unicast transport.
#[cfg(feature = "transport_multilink")]
pub(crate) mod multilink;
/// Negotiates protocol patch settings during unicast establishment.
pub(crate) mod patch;
/// Negotiates quality-of-service settings during unicast establishment.
pub(crate) mod qos;
/// Negotiates shared-memory region names during unicast establishment.
pub(crate) mod region_name;
/// Organizes the shm subsystem and declares its child modules.
#[cfg(feature = "shared-memory")]
pub mod shm;
