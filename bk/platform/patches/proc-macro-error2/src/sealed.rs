//! Sealed marker traits used to restrict implementations of diagnostic extension APIs.
//!
//! TODO Reviewed ??-??-2026.

/// Defines the operations required for sealed behavior.
pub trait Sealed {}

impl Sealed for crate::Diagnostic {}
