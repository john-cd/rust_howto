//! Errors of the tasks.
//!
//! TODO Reviewed ??-??-2026

use std::fmt::Display;
use std::fmt::Formatter;

/// The reasons why a task failed.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum TaskError {
    /// The task was interrupted.
    Interrupted,
    /// Any other error.
    Other,
}

impl Display for TaskError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        // Just call `Debug`.
        write!(f, "{self:?}")?;
        Ok(())
    }
}

impl std::error::Error for TaskError {}
