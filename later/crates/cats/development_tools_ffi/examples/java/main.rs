//! Compiles the Java JNI example on any supported Rust target.
//!
//! TODO Reviewed ??-??-2026.

#[cfg(feature = "java")]
mod jni;

/// Provides the binary target required to compile the JNI example module.
fn main() {
    // The exported JNI function is called by a JVM, not by this binary.
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
