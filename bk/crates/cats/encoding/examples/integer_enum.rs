// TODO Reviewed ??-??-2026.
//! Serializes an enum using its protocol's assigned integer codes.

// ANCHOR: example
use serde_repr::Deserialize_repr;
use serde_repr::Serialize_repr;

/// Status codes assigned by the protocol.
#[derive(Debug, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
enum Status {
    Ready = 10,
    Busy = 20,
}

/// Checks the numeric representation and rejects an unassigned code.
///
/// # Errors
///
/// Returns an error if Serde cannot serialize or deserialize an assigned code.
fn main() -> serde_json::Result<()> {
    for (status, code) in [(Status::Ready, "10"), (Status::Busy, "20")] {
        assert_eq!(serde_json::to_string(&status)?, code);
        assert_eq!(serde_json::from_str::<Status>(code)?, status);
    }
    assert!(serde_json::from_str::<Status>("11").is_err());
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    /// Round-trips assigned codes and rejects an unknown code.
    #[test]
    fn checks_protocol_codes() -> serde_json::Result<()> {
        super::main()
    }
}
