//! Organizes the shm subsystem and declares its child modules.
//! Parent components use this module to access the subsystem functionality.
//!
//! TODO Reviewed ??-??-2026.

// Copyright (c) 2026 ZettaScale Technology
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
/// Transfers shared-memory payloads between transport and application without copying their
/// contents.
pub mod handoff;
/// Converts shared-memory metadata between the transport and interoperable representations.
pub mod interop;
/// Creates and tracks the shared-memory context attached to a transport.
pub mod shm_context;

pub use interop::ProviderInitState;
