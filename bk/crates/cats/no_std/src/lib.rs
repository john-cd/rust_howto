//! A checksum library that supports targets without the standard library.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: library
#![cfg_attr(not(feature = "std"), no_std)]

/// Wrapping byte arithmetic without allocation or operating-system services.
mod checksum;

pub use checksum::checksum;
// ANCHOR_END: library
