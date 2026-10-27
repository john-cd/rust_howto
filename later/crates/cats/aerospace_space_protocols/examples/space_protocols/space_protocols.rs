#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: ccsds-space-packet
/// Serializes a CCSDS packet and checks its primary header and payload.
fn ccsds_space_packet() -> [u8; 8] {
    use arbitrary_int::u11;
    use arbitrary_int::u14;
    use spacepackets::CcsdsPacket as _;
    use spacepackets::PacketType;
    use spacepackets::SequenceFlags;
    use spacepackets::SpHeader;

    let payload = [0xCA, 0xFE];
    let header = SpHeader::new_for_tm(
        u11::new(0x10),
        SequenceFlags::Unsegmented,
        u14::new(1),
        payload.len() as u16 - 1,
    );
    let mut packet = [0_u8; 8];
    let payload_start = header
        .write_to_be_bytes(&mut packet)
        .expect("CCSDS header should fit in the packet buffer");
    payload_start[..payload.len()].copy_from_slice(&payload);

    let (parsed_header, parsed_payload) = SpHeader::from_be_bytes(&packet)
        .expect("serialized CCSDS header should parse");
    assert_eq!(parsed_header.apid().value(), 0x10);
    assert_eq!(parsed_header.seq_count().value(), 1);
    assert_eq!(parsed_header.packet_type(), PacketType::Tm);
    assert_eq!(parsed_payload, payload);
    packet
}
// ANCHOR_END: ccsds-space-packet

/// Runs the packet construction examples.
fn main() {
    let packet = ccsds_space_packet();
    println!("CCSDS packet: {packet:02X?}");
    crate::pus_telecommand::pus_telecommand();
    crate::pus_telemetry::pus_telemetry();
}

/// Runs the space protocols examples through the chapter entry point.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the space protocols example through its entry point.
    #[test]
    fn test_space_protocols_example_entry_point() {
        main();
    }

    /// Checks that a CCSDS packet preserves its application identifier and
    /// sequence count.
    #[test]
    fn ccsds_packet_round_trips_header_and_payload() {
        let packet = ccsds_space_packet();
        assert_eq!(&packet[6..], &[0xCA, 0xFE]);
    }

    /// Checks that a PUS telecommand preserves its service and payload fields.
    #[test]
    fn pus_telecommand_round_trips_service_and_payload() {
        crate::pus_telecommand::pus_telecommand();
    }
}
