// ANCHOR: example
//! This example demonstrates conditional compilation using the `#[cfg]`
//! attribute and `cfg!` macro.
//!
//! Conditional compilation allows you to include or exclude code based on
//! certain conditions, such as the target operating system, architecture,
//! or enabled features.

/// This function only gets compiled
/// if the target OS is Linux.
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running Linux!");
}

/// This function only gets compiled
/// if the target OS is *not* Linux.
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running Linux!");
}

fn main() {
    are_you_on_linux();

    println!("Are you sure?");
    // Alternative to `#[cfg(...)]`: use `cfg!(...)`.
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely Linux!");
    } else {
        println!("Yes. It's definitely *not* Linux!");
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
