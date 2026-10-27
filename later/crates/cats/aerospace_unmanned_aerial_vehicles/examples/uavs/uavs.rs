#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Receiving telemetry from a Tello drone.

use tello::Drone;
use tello::Message;
use tello::Package;
use tello::PackageData;
use tello::ResponseMsg;

/// Connects to a drone and reads its flight data.
fn connect_drone() -> Result<(), String> {
    let mut drone = Drone::new("192.168.10.1:8889");
    drone.connect(11111);
    if let Some(msg) = drone.poll() {
        match msg {
            Message::Data(Package {
                data: PackageData::FlightData(d),
                ..
            }) => {
                println!("battery {}", d.battery_percentage);
            }
            Message::Response(ResponseMsg::Connected(_)) => {
                println!("connected");
                drone
                    .throw_and_go()
                    .expect("the uavs example invariant should hold");
            }
            _ => (),
        }
    }
    Ok(())
}

/// Calls [`connect_drone`] and reports the error, if any.
fn main() {
    let _ = connect_drone();
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the uavs example through its entry point.
    #[test]
    #[ignore = "requires real drone"]
    fn test_uavs_example_entry_point() {
        main();
    }
}
// TODO
