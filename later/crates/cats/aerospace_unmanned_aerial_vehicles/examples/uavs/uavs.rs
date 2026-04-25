#![allow(dead_code)]
use std::time::Duration;

use tello::Drone;
use tello::Message;
use tello::Package;
use tello::PackageData;
use tello::ResponseMsg;

// ANCHOR: example
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
                drone.throw_and_go().unwrap();
            }
            _ => (),
        }
    }
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
    Ok(())
}
// ANCHOR_END: example

fn main() {}

#[test]
#[ignore = "requires real drone"]
fn test() {
    let _ = connect_drone();
}
