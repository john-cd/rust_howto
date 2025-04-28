// ANCHOR: example
use cfg_if::cfg_if;

/// This example demonstrates how to use `cfg-if` to conditionally compile
/// code blocks.
///
/// The `cfg-if` crate provides a convenient way to handle conditional
/// compilation based on configuration flags. This is particularly useful
/// when you need to write code that behaves differently depending on the
/// target platform, enabled features, or other build-time configurations.
///
/// First, add the dependency to your Cargo.toml:
/// ```toml
/// [dependencies]
/// cfg-if = "1.0.0"
/// ```
fn main() {
    // Basic usage example:
    cfg_if! {
        if #[cfg(target_os = "windows")] {
            println!("Running on Windows!");
        } else if #[cfg(target_os = "macos")] {
            println!("Running on macOS!");
        } else if #[cfg(target_os = "linux")] {
            println!("Running on Linux!");
        } else {
            println!("Running on an unknown platform!");
        }
    }

    // Another example with feature flags:
    cfg_if! {
        if #[cfg(feature = "full")] {
            fn get_functionality() -> &'static str {
                "Full functionality enabled"
            }
        } else {
            fn get_functionality() -> &'static str {
                "Limited functionality only"
            }
        }
    }

    println!("Functionality: {}", get_functionality());

    // Nested example.
    cfg_if! {
        if #[cfg(target_arch = "x86_64")] {
            cfg_if! {
                if #[cfg(target_feature = "avx2")] {
                    println!("Using AVX2 optimized routines");
                } else if #[cfg(target_feature = "sse2")] {
                    println!("Using SSE2 optimized routines");
                } else {
                    println!("Using standard routines");
                }
            }
        } else {
            println!("Architecture-specific optimizations not available");
        }
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
