//! Organizes the common subsystem and declares its child modules.
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
/// Batches transport messages and controls when queued messages are flushed.
pub mod batch;
/// Reassembles fragmented transport messages and enforces fragment limits.
pub(crate) mod defragmentation;
/// Coordinates bounded transport read and write stages, including their wakeups and backoff.
pub(crate) mod pipeline;
/// Tracks sequence numbers and queues for transport priorities.
pub(crate) mod priority;
/// Defines sequence-number arithmetic and resolution handling for transport messages.
pub(crate) mod seq_num;
/// Organizes the shm subsystem and declares its child modules.
#[cfg(feature = "shared-memory")]
pub mod shm;
