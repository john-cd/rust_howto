// TODO Reviewed ??-??-2026.
//! Runnable entry point for the peripherals example.
/// Provides list serial ports behavior for the peripherals component.
mod list_serial_ports;

/// Runs the example demonstrated by this module.
fn main() {
    list_serial_ports::run();
}
