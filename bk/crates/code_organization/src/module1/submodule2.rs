// TODO Reviewed ??-??-2026.
//! Library data types.

/// Contains the name for a greeting.
pub struct Greeting<'a>(
    /// Borrowed greeting text returned without allocating a new string.
    pub &'a str,
);
