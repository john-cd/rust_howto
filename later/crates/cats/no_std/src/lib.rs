//! A checksum library that supports targets without the standard library.

#![cfg_attr(not(feature = "std"), no_std)]

mod checksum;

pub use checksum::checksum;
