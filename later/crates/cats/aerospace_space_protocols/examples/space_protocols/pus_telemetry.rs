// ANCHOR: pus-telemetry
//! Create and read a PUS-C connection-test telemetry packet.

use arbitrary_int::u11;
use spacepackets::SpHeader;
use spacepackets::ecss::tm::CreatorConfig;
use spacepackets::ecss::tm::MessageTypeId;
use spacepackets::ecss::tm::PusTmCreator;
use spacepackets::ecss::tm::PusTmReader;
use spacepackets::ecss::tm::PusTmSecondaryHeader;

/// Serializes a service 17 telemetry response and checks the decoded fields.
///
/// Telemetry is data flowing from the spacecraft to the ground. This packet is
/// the spacecraft's answer to the connection-test telecommand (service 17,
/// subtype 1): service 17, subtype 2 means "connection test report".
pub fn pus_telemetry() {
    // Telemetry carries the time it was generated. This fixed seven-byte
    // timestamp makes the example deterministic.
    let timestamp = [0_u8; 7];
    let telemetry = PusTmCreator::new_no_source_data(
        SpHeader::new_from_apid(u11::new(0x42)),
        PusTmSecondaryHeader::new_simple(MessageTypeId::new(17, 2), &timestamp),
        CreatorConfig::default(),
    );
    let mut bytes = [0_u8; 32];
    let length = telemetry
        .write_to_bytes(&mut bytes)
        .expect("the telemetry packet should fit in the buffer");
    let decoded = PusTmReader::new(&bytes[..length], timestamp.len())
        .expect("the serialized telemetry packet should parse");
    assert_eq!(decoded.apid().value(), 0x42);
    assert_eq!(decoded.service_type_id(), 17);
    assert_eq!(decoded.message_subtype_id(), 2);
    assert_eq!(decoded.timestamp(), &timestamp);
}
// ANCHOR_END: pus-telemetry

#[cfg(test)]
mod tests {
    /// Checks a PUS telemetry packet survives serialization and decoding.
    #[test]
    fn telemetry_round_trips() {
        super::pus_telemetry();
    }
}
