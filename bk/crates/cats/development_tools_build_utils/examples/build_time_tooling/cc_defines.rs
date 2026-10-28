// TODO Reviewed ??-??-2026.

//! Compatibility entry point for the C preprocessor definitions example.

/// Runs the linked C example through the original command name.
///
/// # Errors
///
/// Returns an error if a C string is not valid UTF-8.
pub fn run() -> anyhow::Result<()> {
    super::cc_defines1::run()
}
