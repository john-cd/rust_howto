// ANCHOR: pus-telecommand
//! Create and parse a PUS-C telecommand.
/// Creates, serializes, and parses a PUS-C connection-test telecommand.
///
/// A telecommand is an instruction sent from the ground to a spacecraft.
/// PUS (Packet Utilization Standard, from ECSS) layers named services on top
/// of CCSDS packets. Service 17 is the "test" service: subtype 1 asks the
/// spacecraft "are you alive?" and the spacecraft answers with a telemetry
/// packet (subtype 2), like a ping.
pub fn pus_telecommand() {
    use arbitrary_int::u11;
    use arbitrary_int::u14;
    use spacepackets::CcsdsPacket as _;
    use spacepackets::SpHeader;
    use spacepackets::ecss::CreatorConfig;
    use spacepackets::ecss::PusPacket as _;
    use spacepackets::ecss::tc::MessageTypeId;
    use spacepackets::ecss::tc::PusTcCreator;
    use spacepackets::ecss::tc::PusTcReader;

    // Optional application data; a plain connection test does not need any.
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
