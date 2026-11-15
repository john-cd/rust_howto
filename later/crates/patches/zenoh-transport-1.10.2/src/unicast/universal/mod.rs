//! Organizes the universal subsystem and declares its child modules.
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
/// Tracks the state of a universal unicast transport and its peer.
pub(crate) mod transport;

/// Provides link operations shared by universal unicast transports.
mod link;
/// Receives, orders, and dispatches messages for universal unicast transport.
mod rx;
/// Schedules and sends messages for universal unicast transport.
mod tx;
