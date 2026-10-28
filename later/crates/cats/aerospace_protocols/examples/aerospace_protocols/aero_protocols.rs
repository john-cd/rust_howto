// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Encode and decode a MAVLink heartbeat without a vehicle or network.

use std::error::Error;

use mavlink::MavHeader;
use mavlink::MavlinkReader;
use mavlink::MavlinkVersion;
use mavlink::dialects::common;

/// Encodes a heartbeat and reads it back from an in-memory MAVLink 2 frame.
fn round_trip()
-> Result<(MavHeader, common::MavMessage), Box<dyn Error + Send + Sync>> {
    let header = MavHeader {
        system_id: 1,
        component_id: 1,
        sequence: 7,
    };
    let message = common::MavMessage::HEARTBEAT(common::HEARTBEAT_DATA {
        custom_mode: 42,
        mavtype: common::MavType::MAV_TYPE_QUADROTOR,
        autopilot: common::MavAutopilot::MAV_AUTOPILOT_ARDUPILOTMEGA,
        base_mode: common::MavModeFlag::MAV_MODE_FLAG_SAFETY_ARMED,
        system_status: common::MavState::MAV_STATE_STANDBY,
        mavlink_version: 3,
    });
    let mut bytes = Vec::new();
    mavlink::write_v2_msg(&mut bytes, header, &message)?;
    let mut reader = MavlinkReader::new(bytes.as_slice());
    Ok(reader.read_message::<common::MavMessage>(MavlinkVersion::V2)?)
}

/// Prints the decoded heartbeat and its frame header.
fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (header, message) = round_trip()?;
    println!("{header:?}: {message:?}");
    Ok(())
}
// ANCHOR_END: example

/// Runs the in-memory heartbeat example.
pub fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks the decoded header and heartbeat fields after serialization.
    #[test]
    fn heartbeat_round_trips_through_a_mavlink_frame()
    -> Result<(), Box<dyn Error + Send + Sync>> {
        let (header, message) = round_trip()?;
        assert_eq!(header.system_id, 1);
        assert_eq!(header.component_id, 1);
        assert_eq!(header.sequence, 7);
        let common::MavMessage::HEARTBEAT(heartbeat) = message else {
            return Err("expected a heartbeat message".into());
        };
        assert_eq!(heartbeat.custom_mode, 42);
        assert_eq!(heartbeat.mavtype, common::MavType::MAV_TYPE_QUADROTOR);
        assert_eq!(
            heartbeat.system_status,
            common::MavState::MAV_STATE_STANDBY
        );
        Ok(())
    }
}
