// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Fill a byte array with randomness from the operating system.

/// Generates and displays a sample nonce using the OS random source.
fn main() -> anyhow::Result<()> {
    let mut nonce = [0_u8; 12];
    getrandom::fill(&mut nonce)?;

    println!("sample nonce: {}", hex::encode(nonce));
    Ok(())
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the example can fill its nonce buffer.
    #[test]
    fn test_main() -> anyhow::Result<()> {
        main()
    }
}
