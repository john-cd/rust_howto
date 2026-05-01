#![allow(dead_code)]
// ANCHOR: example
//! A Rust example using the `nix` crate to interface with Linux APIs.
//!
//! This example prints the current process ID, parent process ID, and the
//! operating system information obtained from `uname`.

#[cfg(target_os = "linux")]
fn main() {
    use nix::sys::utsname::uname;
    use nix::unistd::getpid;
    use nix::unistd::getppid;

    let uname = uname();
    let pid = getpid();
    let ppid = getppid();

    println!("Linux system information:");
    println!("  sysname: {:?}", uname.sysname());
    println!("  nodename: {:?}", uname.nodename());
    println!("  release: {:?}", uname.release());
    println!("  version: {:?}", uname.version());
    println!("  machine: {:?}", uname.machine());
    println!("");
    println!("Process information:");
    println!("  pid: {}", pid);
    println!("  ppid: {}", ppid);
}

#[cfg(not(target_os = "linux"))]
fn main() {
    println!(
        "This example is intended to demonstrate Linux API interop and is only supported on Linux."
    );
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        main();
    }
}
