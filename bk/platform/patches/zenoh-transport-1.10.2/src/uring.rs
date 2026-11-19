//! Uses Linux io_uring to drive supported transport operations.
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

use std::cmp::max;

use zenoh_result::ZResult;
use zenoh_uring::api::reader::Reader;
use zenoh_uring::api::types::BufferCount;

/// Data for uring operations, including reader.
#[derive(Clone)]
pub struct Uring {
    /// Reader used to access the decoded Capn Proto message.
    pub reader: Reader,
}

impl Uring {
    /// Creates a new uring with the supplied configuration.
    pub fn new(batch_size: usize, link_rx_buffer_size: usize) -> ZResult<Self> {
        // add 2 bytes for size header in case of streamed links
        let batch_size = batch_size + (u16::BITS / 8) as usize;
        let batch_count = max(link_rx_buffer_size / batch_size, 16) as BufferCount;

        let reader = Reader::new(batch_size, batch_count)?;
        Ok(Self { reader })
    }
}
