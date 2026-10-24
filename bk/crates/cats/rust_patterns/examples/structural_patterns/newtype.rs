// ANCHOR: example
//! Uses a newtype so user identifiers cannot be confused with arbitrary
//! integers.

/// A nonzero identifier for a user.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct UserId(u64);

impl UserId {
    /// Creates an identifier and rejects the reserved zero value.
    fn new(value: u64) -> anyhow::Result<Self> {
        anyhow::ensure!(value != 0, "user identifiers must be nonzero");
        Ok(Self(value))
    }

    /// Returns the integer representation of the identifier.
    fn get(self) -> u64 {
        self.0
    }
}

/// Prints a user identifier constructed through its invariant-checking API.
fn main() -> anyhow::Result<()> {
    let user_id = UserId::new(42)?;
    println!("User id: {}", user_id.get());
    Ok(())
}
// ANCHOR_END: example

/// Runs the newtype example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::UserId;

    /// Verifies a user identifier rejects the reserved zero value.
    #[test]
    fn user_id_rejects_zero() {
        assert!(UserId::new(0).is_err());
    }
}
