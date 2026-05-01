#![allow(dead_code)]
// ANCHOR: example
//! A FreeBSD-specific example using the `nix` crate to call FreeBSD APIs.
//!
//! This example prints operating system information from `uname` and the
//! current process ID / parent process ID.

#[cfg(target_os = "freebsd")]
fn main() {
    use nix::sys::utsname::uname;
    use nix::unistd::getpid;
    use nix::unistd::getppid;

    let uname = uname();
    let pid = getpid();
    let ppid = getppid();

    println!("FreeBSD system information:");
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

#[cfg(not(target_os = "freebsd"))]
fn main() {
    println!(
        "This example is intended for FreeBSD and is only supported on FreeBSD."
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
// TODO review
