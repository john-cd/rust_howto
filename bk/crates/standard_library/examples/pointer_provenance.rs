//! A borrowed allocation passed across an `extern "C"` function boundary.

// ANCHOR: example
/// # Safety
/// `bytes` must point to `len` initialized bytes that remain readable for this call.
unsafe extern "C" fn second_byte(bytes: *const u8, len: usize) -> u16 {
    if len < 2 {
        return 256;
    }

    // SAFETY: the caller guarantees that bytes 0..len are readable. `with_addr`
    // derives the new pointer from `bytes` and keeps its provenance.
    let second = bytes.with_addr(bytes.addr() + 1);
    u16::from(unsafe { *second })
}

fn read_second(bytes: &[u8]) -> Option<u8> {
    // SAFETY: the slice provides an initialized, readable allocation for the
    // duration of the call. The callee checks the length before dereferencing.
    u8::try_from(unsafe { second_byte(bytes.as_ptr(), bytes.len()) }).ok()
}
// ANCHOR_END: example

fn main() {
    assert_eq!(read_second(b"Rust"), Some(b'u'));
    assert_eq!(read_second(b"R"), None);
}

#[cfg(test)]
mod tests {
    use super::read_second;

    #[test]
    fn reads_only_within_the_live_slice() {
        assert_eq!(read_second(&[]), None);
        assert_eq!(read_second(&[1]), None);
        assert_eq!(read_second(&[1, 2]), Some(2));
    }
}
