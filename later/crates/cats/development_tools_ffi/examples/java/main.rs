//! Compiles the Java JNI example on any supported Rust target.
//!
//! TODO Reviewed ??-??-2026.

/// Calling Rust from Java with the Java Native Interface.
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

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
