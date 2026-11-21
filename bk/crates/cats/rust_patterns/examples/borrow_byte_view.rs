// TODO Reviewed ??-??-2026.
//! Borrows a typed view over the original byte slice.

// ANCHOR: example
use ref_cast::RefCast;

/// A borrowed packet view that accepts every byte slice, including an empty one.
#[derive(RefCast)]
#[repr(transparent)]
struct PacketBytes([u8]);

impl PacketBytes {
    /// Checks whether the packet begins with its two-byte signature.
    fn has_signature(&self) -> bool {
        self.0.starts_with(b"PK")
    }
}

/// Borrows a packet view and checks that it still points to the original bytes.
fn main() {
    let raw = b"PKdata";
    let packet = PacketBytes::ref_cast(raw.as_slice());

    assert!(packet.has_signature());
    assert!(std::ptr::eq(&packet.0, raw.as_slice()));
    assert!(!PacketBytes::ref_cast(&[]).has_signature());
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    /// Checks the view keeps the same backing bytes and accepts empty input.
    #[test]
    fn borrows_without_copying() {
        super::main();
    }
}
