// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Receiving telemetry from a Tello drone.
//!
//! Decoding a message into text is a pure function, so it can be tested
//! without a drone; only `run_live` talks to the network.
//!
//! A UAV (unmanned aerial vehicle) is the formal name for a drone. Besides
//! accepting commands, a drone continuously reports telemetry, meaning
//! measurements such as battery charge and height. Ground software uses them
//! to display status and to decide when to land.

use anyhow::Result;
use tello::Drone;
use tello::Message;
use tello::Package;
use tello::PackageData;
use tello::ResponseMsg;

/// Describes the messages this example cares about; ignores the others.
fn describe(message: &Message) -> Option<String> {
    match message {
        Message::Data(Package {
            data: PackageData::FlightData(data),
            ..
        }) => Some(format!(
            "battery {}%, height {} dm",
            data.battery_percentage, data.height
        )),
        Message::Response(ResponseMsg::Connected(_)) => Some("connected".to_owned()),
        _ => None,
    }
}

/// Connects to a drone and prints one telemetry or connection message.
pub fn run_live() {
    // The drone's fixed address and command port on its own Wi-Fi network.
    let mut drone = Drone::new("192.168.10.1:8889");
    // The drone sends its replies and video to this local UDP port.
    drone.connect(11111);
    if let Some(message) = drone.poll()
        && let Some(text) = describe(&message)
    {
        println!("{text}");
    }
}

/// Builds a flight-data packet with the supplied height and battery level.
#[expect(
    clippy::little_endian_bytes,
    reason = "the Tello protocol is little-endian"
)]
fn flight_data_packet(height: u8, battery: u8) -> Vec<u8> {
    let mut data = [0_u8; 24];
    data[0] = height;
    data[12] = battery;
    // The payload plus a two-byte CRC is `size`; the length field holds
    // `(size + 11) << 3`.
    let size = u16::try_from(data.len() + 2).expect("the size fits");
    let length = (size + 11) << 3;
    let mut packet = vec![0xcc];
    packet.extend_from_slice(&length.to_le_bytes());
    packet.push(0); // header CRC (not checked by the decoder)
    packet.push(0x68); // packet type
    packet.extend_from_slice(&0x0056_u16.to_le_bytes()); // flight message
    packet.extend_from_slice(&1_u16.to_le_bytes()); // sequence number
    packet.extend_from_slice(&data);
    packet.extend_from_slice(&[0, 0]); // payload CRC (not checked)
    packet
}

/// Decodes a synthetic flight-data packet and prints the status.
///
/// # Errors
///
/// Returns an error if the example packet cannot be decoded.
pub fn run() -> Result<()> {
    let message = Message::try_from(flight_data_packet(12, 87))
        .map_err(|error| anyhow::anyhow!("could not decode flight data: {error:?}"))?;
    let text = describe(&message).ok_or_else(|| anyhow::anyhow!("expected flight data"))?;
    println!("{text}");
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// A captured-style flight-data packet decodes into the expected text.
    #[test]
    fn flight_data_is_described() {
        let message =
            Message::try_from(flight_data_packet(12, 87)).expect("the packet is well formed");
        assert_eq!(
            describe(&message).as_deref(),
            Some("battery 87%, height 12 dm")
        );
    }

    /// The connection acknowledgement is recognized.
    #[test]
    fn connection_acknowledgement_is_described() {
        let message = Message::try_from(b"conn_ack:\x01\x02".to_vec())
            .expect("the acknowledgement is well formed");
        assert_eq!(describe(&message).as_deref(), Some("connected"));
    }

    /// Other messages are ignored.
    #[test]
    fn other_messages_are_ignored() {
        assert_eq!(describe(&Message::Frame(0, Vec::new())), None);
    }

    /// Replays a flight-data fixture through the offline entry point.
    #[test]
    fn offline_demo_runs() -> Result<()> {
        run()
    }

    /// Verifies that the `uavs` example runs.
    #[test]
    #[ignore = "requires real drone"]
    fn live_telemetry_poll() {
        run_live();
    }
}
