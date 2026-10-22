//! Runs the Java example on non-Windows platforms.
//!
//! TODO Reviewed ??-??-2026

#[cfg(not(windows))] // TODO review Windows support for Java FFI examples
mod jni;

/// Runs the example if it is supported.
fn main() {
    #[cfg(not(windows))]
    jni::run();
}
