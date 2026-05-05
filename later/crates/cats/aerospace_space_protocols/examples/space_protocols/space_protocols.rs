#![allow(dead_code)]
// ANCHOR: example
use arbitrary_int::u11;
use arbitrary_int::u14;
use spacepackets::CcsdsPacket;
use spacepackets::SequenceFlags;
use spacepackets::SpHeader;

fn main() {
    let sp_header = SpHeader::new_for_unseg_tc(u11::new(0x42), u14::new(12), 1);
    println!("{:?}", sp_header);
    let mut ccsds_buf: [u8; 32] = [0; 32];
    sp_header
        .write_to_be_bytes(&mut ccsds_buf)
        .expect("Writing CCSDS TC header failed");
    println!("{:x?}", &ccsds_buf[0..6]);

    let tm_header = SpHeader::new_for_tm(
        u11::new(0x10),
        SequenceFlags::Unsegmented,
        u14::new(0x01),
        3,
    );
    println!("TM header: {:?}", tm_header);
    assert_eq!(tm_header.apid().value(), 0x10);
    assert_eq!(tm_header.seq_count().value(), 0x01);
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_main() {
        main();
    }
}
// TODO review the space_protocols crate and add more examples for telemetry,
// telecommand, and file transfer protocols.
