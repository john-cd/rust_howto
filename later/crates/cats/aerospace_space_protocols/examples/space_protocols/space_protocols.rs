#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: ccsds-space-packet
/// Serializes a CCSDS packet and checks its primary header and payload.
fn ccsds_space_packet() -> [u8; 8] {
    use arbitrary_int::u11;
    use arbitrary_int::u14;
    use spacepackets::CcsdsPacket as _;
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
    assert_eq!(parsed_payload, payload);
    packet
}
// ANCHOR_END: ccsds-space-packet

// ANCHOR: pus-telecommand
/// Creates, serializes, and parses a PUS-C connection-test telecommand.
fn pus_telecommand() {
    use arbitrary_int::u11;
    use arbitrary_int::u14;
    use spacepackets::CcsdsPacket as _;
    use spacepackets::SpHeader;
    use spacepackets::ecss::CreatorConfig;
    use spacepackets::ecss::PusPacket as _;
    use spacepackets::ecss::tc::MessageTypeId;
    use spacepackets::ecss::tc::PusTcCreator;
    use spacepackets::ecss::tc::PusTcReader;

    let app_data = [0xCA, 0xFE];
    let telecommand = PusTcCreator::new_simple(
        SpHeader::new_for_unseg_tc(u11::new(0x42), u14::new(12), 0),
        MessageTypeId::new(17, 1),
        &app_data,
        CreatorConfig::default(),
    );
    let mut packet = [0_u8; 32];
    let packet_len = telecommand
        .write_to_bytes(&mut packet)
        .expect("PUS telecommand should fit in the packet buffer");
    let parsed = PusTcReader::new(&packet[..packet_len])
        .expect("serialized PUS telecommand should parse");

    assert_eq!(parsed.apid().value(), 0x42);
    assert_eq!(parsed.seq_count().value(), 12);
    assert_eq!(parsed.service_type_id(), 17);
    assert_eq!(parsed.message_subtype_id(), 1);
    assert_eq!(parsed.user_data(), app_data);
}
// ANCHOR_END: pus-telecommand

/// Runs both packet construction examples.
fn main() {
    let packet = ccsds_space_packet();
    println!("CCSDS packet: {packet:02X?}");
    pus_telecommand();
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
        pus_telecommand();
    }
}
// TODO review the space_protocols crate and add more examples for telemetry,
// telecommand, and file transfer protocols.
