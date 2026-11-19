//! Build and parse CCSDS space packets and ECSS PUS packets.
//!
//! Spacecraft and ground stations exchange small binary packets over radio
//! links. CCSDS defines the basic space packet, and the ECSS PUS standard
//! defines named services, such as a connection test, on top of it. This
//! example creates each kind of packet, serializes it to bytes and parses it
//! back.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: ccsds-space-packet

/// Serializes a CCSDS packet and checks its primary header and payload.
///
/// CCSDS (Consultative Committee for Space Data Systems) is the standards
/// body that defines how spacecraft and ground stations exchange data. Its
/// "space packet" is a small binary container: a six-byte header followed by
/// the payload. The header holds an application identifier (APID), which says
/// which on-board application produced or should receive the packet, and a
/// sequence count, which lets the receiver detect missing packets on a noisy
/// radio link.
fn ccsds_space_packet() -> [u8; 8] {
    use arbitrary_int::u11;
    use arbitrary_int::u14;
    use spacepackets::CcsdsPacket as _;
    use spacepackets::SequenceFlags;
    use spacepackets::SpHeader;

    let payload = [0xCA, 0xFE];
    // `new_for_tm` builds a telemetry packet (spacecraft to ground). The data
    // length field stores the payload length minus one, as the standard
    // requires.
    let header = SpHeader::new_for_tm(
        u11::new(0x10),
        SequenceFlags::Unsegmented,
        u14::new(1),
        payload.len() as u16 - 1,
    );
    let mut packet = [0_u8; 8];
    let header_len = header
        .write_to_slice(&mut packet)
        .expect("CCSDS header should fit in the packet buffer");
    packet[header_len..header_len + payload.len()].copy_from_slice(&payload);

    let (parsed_header, parsed_payload) =
        first_telemetry_packet(&packet).expect("serialized CCSDS packet should parse");
    assert_eq!(parsed_header.apid().value(), 0x10);
    assert_eq!(parsed_header.seq_count().value(), 1);
    assert_eq!(parsed_payload, payload);
    packet
}
/// Reads the first complete telemetry packet in a byte slice.
fn first_telemetry_packet(bytes: &[u8]) -> Option<(spacepackets::SpHeader, &[u8])> {
    use spacepackets::CCSDS_HEADER_LEN;
    use spacepackets::CcsdsPacket as _;
    use spacepackets::PacketType;
    use spacepackets::SpHeader;

    let (header, _) = SpHeader::from_be_bytes(bytes).ok()?;
    if header.ccsds_version().value() != 0 || header.packet_type() != PacketType::Tm {
        return None;
    }
    let payload = bytes.get(CCSDS_HEADER_LEN..header.packet_len())?;
    Some((header, payload))
}
// ANCHOR_END: ccsds-space-packet

/// Prints a CCSDS space packet.
fn main() {
    let packet = ccsds_space_packet();
    println!("CCSDS packet: {packet:02X?}");
}

/// Runs the CCSDS packet demonstration.
pub fn run() {
    main();
}

/// Tests behavior implemented by the surrounding module.
#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the space protocols example through its entry point.
    #[test]
    fn test_space_protocols_example_entry_point() {
        main();
    }

    /// sequence count.
    #[test]
    fn ccsds_packet_round_trips_header_and_payload() {
        let packet = ccsds_space_packet();
        assert_eq!(&packet[6..], &[0xCA, 0xFE]);
    }

    /// Reads one packet from a capture and rejects an incomplete packet.
    #[test]
    fn respects_packet_boundaries() {
        let packet = ccsds_space_packet();
        let capture = [packet, packet].concat();
        assert_eq!(
            first_telemetry_packet(&capture).map(|(_, data)| data),
            Some(&packet[6..]),
        );
        assert!(first_telemetry_packet(&packet[..7]).is_none());
    }

    /// Preserves service and payload fields through PUS telecommand encoding.
    #[test]
    fn pus_telecommand_round_trips_service_and_payload() {
        crate::pus_telecommand::pus_telecommand();
    }
}
