#![allow(dead_code)]
// TODO Reviewed ??-??-2026
// TODO avoid Result<(), String> use anyhow?

// ANCHOR: example
//! Sending commands to a Tello drone.

use tello::Drone;

/// Connects to a drone and sends it commands.
fn main() -> Result<(), String> {
    let mut drone = Drone::new("192.168.10.1:8889");
    drone.connect(11111);
    println!("Drone connected.");

    // Commands to the drone
    drone.take_off().map_err(|_| "Failed to take off")?;
    drone.land().map_err(|_| "Failed to land")?;

    Ok(())
}
// ANCHOR_END: example

pub fn run() -> Result<(), String> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the drones1 example through its entry point.
    #[test]
    #[ignore = "requires real drone"]
    fn test_drones1_example_entry_point() {
        main().expect("example entry point should succeed"); // TODO no unwrap
    }
}
