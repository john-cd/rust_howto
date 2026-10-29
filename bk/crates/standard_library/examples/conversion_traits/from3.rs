// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Parsing from raw bytes.

/// A message, created from raw bytes.
#[derive(Debug)]
struct Message(String);

// Implement `From` to convert from a `u8` slice to our custom struct.
impl From<&[u8]> for Message {
    fn from(bytes: &[u8]) -> Self {
        Self(String::from_utf8_lossy(bytes).into())
    }
}

/// Converts raw bytes into a `Message`.
fn main() {
    let raw: &[u8] = b"Hello world";
    let msg: Message = raw.into(); // Use `From<&[u8]>`.
    println!("{msg:?}");
    println!("{}", msg.0);
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from3_example_entry_point() {
        main();
    }

    #[test]
    fn test_from_valid_utf8_bytes() {
        let msg: Message = b"Hello world".as_slice().into();
        assert_eq!(format!("{msg:?}"), r#"Message("Hello world")"#);
    }

    #[test]
    fn test_from_empty_bytes() {
        let msg = Message::from(&b""[..]);
        assert_eq!(format!("{msg:?}"), r#"Message("")"#);
    }

    #[test]
    fn test_invalid_utf8_is_replaced_not_rejected() {
        let msg = Message::from(&[b'a', 0xff, b'b'][..]);
        assert_eq!(format!("{msg:?}"), "Message(\"a\u{fffd}b\")");
    }
}
