// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Checks IPv4 addresses against a configured subnet with `smoltcp`.
//!
//! `smoltcp` is a TCP/IP network stack that needs neither an operating system
//! nor heap allocation, so it fits on microcontrollers (small chips that run
//! one program, such as in sensors or appliances). A subnet is a block of
//! addresses on the same local network. It is written in CIDR notation:
//! `192.0.2.0/24` means the first 24 bits (`192.0.2`) are fixed and the last 8
//! bits identify the device, giving 256 addresses. A device uses this check to
//! decide whether to send a packet directly or via a router.

use smoltcp::wire::Ipv4Address;
use smoltcp::wire::Ipv4Cidr;

/// Returns the documentation subnet used by the example.
///
/// `192.0.2.0/24` is reserved for examples and documentation (RFC 5737), so
/// it never collides with a real network.
fn documentation_subnet() -> Ipv4Cidr {
    Ipv4Cidr::new(Ipv4Address::new(192, 0, 2, 0), 24)
}

/// Prints whether an example device address belongs to the configured subnet.
fn print_example() {
    let subnet = documentation_subnet();
    let device = Ipv4Address::new(192, 0, 2, 42);
    println!(
        "{device} belongs to {subnet}: {}",
        subnet.contains_addr(&device)
    );
}

/// Runs the subnet check from a standalone entry point.
fn main() {
    print_example();
}
// ANCHOR_END: example

/// Runs the subnet-membership example.
pub fn run() {
    print_example();
}

#[cfg(test)]
mod tests {
    use smoltcp::wire::Ipv4Address;

    use super::documentation_subnet;

    /// Verifies that addresses inside and outside the subnet are distinguished.
    #[test]
    fn subnet_contains_only_its_configured_range() {
        let subnet = documentation_subnet();
        assert!(subnet.contains_addr(&Ipv4Address::new(192, 0, 2, 42)));
        assert!(!subnet.contains_addr(&Ipv4Address::new(192, 0, 3, 1)));
    }

    /// Exercises the example entry point.
    #[test]
    fn test_main_entry_point() {
        super::main();
    }
}
