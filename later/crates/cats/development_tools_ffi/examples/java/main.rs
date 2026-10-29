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

#[cfg(test)]
mod tests {
    use super::*;

    // output-only: the JNI example needs a JVM and the `java` feature.
    #[test]
    fn test_main_entry_point() {
        main();
    }
}
