// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Checks IPv4 addresses against a configured subnet with `smoltcp`.

use smoltcp::wire::Ipv4Address;
use smoltcp::wire::Ipv4Cidr;

/// Returns the documentation subnet used by the example.
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
