// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Encode and decode a MAVLink heartbeat without a vehicle or network.
//!
//! MAVLink is a lightweight binary protocol that drones, rovers and ground
//! control stations use to exchange telemetry and commands, usually over a
//! serial radio link or UDP. A "heartbeat" is a small message each side sends
//! about once per second to announce that it is alive, what kind of vehicle it
//! is and what state it is in.

use std::error::Error;

use mavlink::MavHeader;
use mavlink::MavlinkReader;
use mavlink::MavlinkVersion;
use mavlink::dialects::common;

/// Encodes a heartbeat and reads it back from an in-memory MAVLink 2 frame.
fn round_trip(
    sequence: u8,
) -> Result<(MavHeader, common::MavMessage), Box<dyn Error + Send + Sync>> {
    // The header identifies the sender: the vehicle (system) and the part of
    // it (component), such as the autopilot. The sequence number lets the
    // receiver detect lost frames.
    let header = MavHeader {
        system_id: 1,
        component_id: 1,
        sequence,
    };
    // The `common` dialect is the message set shared by most autopilots.
    let message = common::MavMessage::HEARTBEAT(common::HEARTBEAT_DATA {
        custom_mode: 42,
        mavtype: common::MavType::MAV_TYPE_QUADROTOR,
        autopilot: common::MavAutopilot::MAV_AUTOPILOT_ARDUPILOTMEGA,
        base_mode: common::MavModeFlag::empty(),
        system_status: common::MavState::MAV_STATE_STANDBY,
        mavlink_version: 3,
    });
    // Serialize into a byte vector, which stands in for the radio link.
    let mut bytes = Vec::new();
    mavlink::write_v2_msg(&mut bytes, header, &message)?;
    let mut reader = MavlinkReader::new(bytes.as_slice());
    Ok(reader.read_message::<common::MavMessage>(MavlinkVersion::V2)?)
}

/// Counts frames missing between consecutive headers from one ordered channel.
///
/// The caller supplies one unmixed, ordered capture without repeat frames.
fn missing_frames(previous: MavHeader, next: MavHeader) -> u8 {
    next.sequence
        .wrapping_sub(previous.sequence.wrapping_add(1))
}

/// Prints the decoded heartbeat and counts a gap across sequence wraparound.
fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (header, message) = round_trip(254)?;
    let (next, _) = round_trip(1)?;
    println!("{header:?}: {message:?}");
    println!("Missing frames: {}", missing_frames(header, next));
    Ok(())
}
// ANCHOR_END: example

/// Runs the in-memory heartbeat example.
///
/// # Errors
///
/// Returns an error if a step of the example fails.
pub fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    main()
}

/// Checks the decoded header and heartbeat fields after serialization.
#[cfg(test)]
mod tests {
    use super::*;

    /// Checks the decoded header and heartbeat fields after serialization.
    #[test]
    fn heartbeat_round_trips_through_a_mavlink_frame() -> Result<(), Box<dyn Error + Send + Sync>> {
        let (header, message) = round_trip(7)?;
        assert_eq!(header.system_id, 1);
        assert_eq!(header.component_id, 1);
        assert_eq!(header.sequence, 7);
        let common::MavMessage::HEARTBEAT(heartbeat) = message else {
            return Err("expected a heartbeat message".into());
        };
        assert_eq!(heartbeat.custom_mode, 42);
        assert_eq!(heartbeat.mavtype, common::MavType::MAV_TYPE_QUADROTOR);
        assert_eq!(heartbeat.system_status, common::MavState::MAV_STATE_STANDBY);
        Ok(())
    }

    /// Counts missing numbers even when the one-byte sequence wraps.
    #[test]
    fn counts_gaps_across_wraparound() -> Result<(), Box<dyn Error + Send + Sync>> {
        let (last, _) = round_trip(254)?;
        let (adjacent, _) = round_trip(255)?;
        let (after_wrap, _) = round_trip(1)?;
        assert_eq!(missing_frames(last, adjacent), 0);
        assert_eq!(missing_frames(last, after_wrap), 2);
        Ok(())
    }

    /// Runs the example through its entry point without error.
    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
