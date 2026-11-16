// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Uses a newtype so user identifiers cannot be confused with arbitrary
//! integers.
//!
//! A newtype wraps a primitive in a struct, so a user ID and an order ID are
//! different types. The compiler then stops you passing one where the other is
//! expected, at no run-time cost.

/// A nonzero identifier for a user.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct UserId(u64);

impl UserId {
    /// Creates an identifier and rejects the reserved zero value.
    fn new(value: u64) -> anyhow::Result<Self> {
        anyhow::ensure!(value != 0, "user identifiers must be nonzero");
        Ok(Self(value))
    }

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
///
/// # Errors
///
/// Returns an error if a step of the example fails.
pub fn run() -> anyhow::Result<()> {
    main()
}

/// arbitrary
#[cfg(test)]
mod tests {
    use super::UserId;
    use super::main;

    /// Rejects zero when processing the user id.
    #[test]
    fn user_id_rejects_zero() {
        assert!(UserId::new(0).is_err());
    }

    /// Runs the example through its entry point without error.
    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
