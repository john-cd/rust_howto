// ANCHOR: example

// The `cfg-if` crate is useful for conditional compilation based on configuration flags.

// First, add the dependency to your Cargo.toml:
// [dependencies]
// cfg-if = "1.0.0"

use cfg_if::cfg_if;

fn main() {
    // Basic usage example
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

    // Another example with feature flags
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

    // Nested example
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
