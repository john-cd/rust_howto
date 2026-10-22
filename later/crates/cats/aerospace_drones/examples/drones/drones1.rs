//! Sending commands to a Tello drone.
//!
//! TODO Reviewed ??-??-2026

#![allow(dead_code)]

// ANCHOR: example
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

/// Runs the example.
pub fn run() -> Result<(), String> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "requires real drone"]
    fn test_main() {
        main().unwrap();
    }
}
