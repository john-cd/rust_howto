//! A checksum library that supports targets without the standard library.
//!
//! TODO Reviewed ??-??-2026.

#![cfg_attr(not(feature = "std"), no_std)]

mod checksum;

pub use checksum::checksum;
