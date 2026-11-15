//! Tasks that run on a worker thread and that can be cancelled.
//!
//! TODO Reviewed ??-??-2026.

/// Cancelling tasks with tokens that are checked while the task runs.
mod cancel;

/// Defines failures emitted by cancelable operations and conversions used at component boundaries.
/// Callers match these errors to choose recovery or user-facing diagnostics.
mod errors;

/// A task that reports its progress and can be interrupted.
mod task;

/// A worker thread that runs queued tasks.
mod worker;

pub use cancel::*;
pub use errors::*;
pub use task::*;
pub use worker::*;
