//! Lists the serial ports that the operating system reports.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: example
use serialport::SerialPortInfo;
use serialport::SerialPortType;

/// Describes a port in one line: its name and how it is attached.
fn describe(port: &SerialPortInfo) -> String {
    let kind = match &port.port_type {
        SerialPortType::UsbPort(usb) => {
            format!("USB {:04x}:{:04x}", usb.vid, usb.pid)
        }
        SerialPortType::PciPort => "PCI".to_owned(),
        SerialPortType::BluetoothPort => "Bluetooth".to_owned(),
        SerialPortType::Unknown => "unknown".to_owned(),
    };
    format!("{} ({kind})", port.port_name)
}

/// Prints each serial port. Nothing is opened, so no device is disturbed.
///
/// A machine without serial ports yields an empty list; a platform without
/// enumeration support, or a missing permission, yields an error that the
/// application should report rather than treat as fatal.
fn main() {
    match serialport::available_ports() {
        Ok(ports) if ports.is_empty() => println!("no serial ports found"),
        Ok(ports) => {
            for port in &ports {
                println!("{}", describe(port));
            }
        }
        Err(error) => println!("cannot enumerate serial ports: {error}"),
    }
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use serialport::UsbPortInfo;

    use super::*;

    /// Runs the example through its entry point (output-only: the list
    /// depends on the machine, so there is nothing portable to assert).
    // output-only: the ports depend on the machine running the test.
    #[test]
    fn test_main_entry_point() {
        main();
    }

    /// Verifies a USB port is described with its vendor and product ids.
    #[test]
    fn describes_a_usb_port() {
        let port = SerialPortInfo {
            port_name: "/dev/ttyACM0".to_owned(),
            port_type: SerialPortType::UsbPort(UsbPortInfo {
                vid: 0x2341,
                pid: 0x0043,
                serial_number: None,
                manufacturer: None,
                product: None,
            }),
        };
        assert_eq!(describe(&port), "/dev/ttyACM0 (USB 2341:0043)");
    }

    /// Verifies a port of unknown type is still described by name.
    #[test]
    fn describes_an_unknown_port() {
        let port = SerialPortInfo {
            port_name: "COM3".to_owned(),
            port_type: SerialPortType::Unknown,
        };
        assert_eq!(describe(&port), "COM3 (unknown)");
    }
}
