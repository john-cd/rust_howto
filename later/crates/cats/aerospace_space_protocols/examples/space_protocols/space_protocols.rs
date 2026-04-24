#![allow(dead_code)]
// ANCHOR: example
use arbitrary_int::u11;
use arbitrary_int::u14;
use spacepackets::CcsdsPacket;
use spacepackets::SequenceFlags;
use spacepackets::SpHeader;

pub fn main() {
    let header = SpHeader::new_for_tm(
        u11::new(0x10),
        SequenceFlags::Unsegmented,
        u14::new(0x01),
        3,
    );

    assert_eq!(header.apid().value(), 0x10);
    assert_eq!(header.seq_count().value(), 0x01);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
