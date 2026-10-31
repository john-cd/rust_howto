// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Fill a byte array with randomness from the operating system.
//!
//! Cryptographic keys, nonces and tokens must be unpredictable. `getrandom`
//! asks the operating system for randomness from its secure generator (for
//! example `/dev/urandom` or `BCryptGenRandom`), which is the right source for
//! secrets.

/// Generates and displays a sample nonce using the OS random source.
fn main() -> anyhow::Result<()> {
    let mut nonce = [0_u8; 12];
    getrandom::fill(&mut nonce)?;

    println!("sample nonce: {}", hex::encode(nonce));
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the example can fill its nonce buffer.
    /// Runs the getrandom example example through its entry point.
    #[test]
    fn test_getrandom_example_example_entry_point() -> anyhow::Result<()> {
        main()
    }

    #[test]
    fn fills_distinct_nonzero_buffers() -> anyhow::Result<()> {
        let mut a = [0_u8; 32];
        let mut b = [0_u8; 32];
        getrandom::fill(&mut a)?;
        getrandom::fill(&mut b)?;
        assert_ne!(a, [0_u8; 32]);
        assert_ne!(a, b);
        getrandom::fill(&mut [])?;
        Ok(())
    }

    #[test]
    fn hex_encoding_of_a_nonce_has_twice_the_length() {
        assert_eq!(hex::encode([0xab, 0x01]), "ab01");
        assert_eq!(hex::encode([0_u8; 12]).len(), 24);
    }
}
