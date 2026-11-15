//! Sealed marker traits used to restrict implementations of diagnostic extension APIs.
//!
//! TODO Reviewed ??-??-2026.

pub trait Sealed {}

impl Sealed for crate::Diagnostic {}
