//! Compiles the Java JNI example on any supported Rust target.
//!
//! TODO Reviewed ??-??-2026.

#[cfg(feature = "java")]
mod jni;

/// Provides the binary target required to compile the JNI example module.
fn main() {
    #[cfg(feature = "java")]
    jni::run();
}
