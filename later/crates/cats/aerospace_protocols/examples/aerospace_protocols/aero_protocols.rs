use mavlink::common;

// ANCHOR: example
pub fn main() {
    // Create a new heartbeat message using the `common` dialect
    let heartbeat = common::HEARTBEAT_DATA {
        custom_mode: 0,
        mavtype: common::MavType::MAV_TYPE_QUADROTOR,
        autopilot: common::MavAutopilot::MAV_AUTOPILOT_ARDUPILOTMEGA,
        base_mode: common::MavModeFlag::MAV_MODE_FLAG_SAFETY_ARMED,
        system_status: common::MavState::MAV_STATE_STANDBY,
        mavlink_version: 3,
    };

    // Create a generic message from the heartbeat data
    let message = common::MavMessage::HEARTBEAT(heartbeat);

    // In a real application, you would send this message over a connection:
    // let mut conn = mavlink::connect("udpin:0.0.0.0:14550").unwrap();
    // let header = mavlink::MavHeader {
    //     system_id: 1,
    //     component_id: 1,
    //     sequence: 0,
    // };
    // conn.send(&header, &message).unwrap();

    // Verify the message type
    assert!(matches!(message, common::MavMessage::HEARTBEAT(_)));
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        main();
    }
}
