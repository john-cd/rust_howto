// TODO Reviewed ??-??-2026.

// ANCHOR: example
use anyhow::Context as _;
use solana_pubkey::Pubkey;

/// Derives a program address for the given program and record.
///
/// # Errors
///
/// Returns an error if the seeds cannot produce a valid address.
#[expect(
    clippy::little_endian_bytes,
    reason = "the seed layout uses a stable little-endian record ID"
)]
fn derive_record_address(program: &Pubkey, record: u64) -> anyhow::Result<(Pubkey, u8)> {
    let record = record.to_le_bytes();
    Pubkey::try_find_program_address(&[b"record", &record], program)
        .context("could not derive a Solana program address")
}

/// Prints a deterministic address and verifies it with its bump seed.
///
/// # Errors
///
/// Returns an error if the seed derivation or verification fails.
#[expect(
    clippy::little_endian_bytes,
    reason = "the seed layout uses a stable little-endian record ID"
)]
pub fn run() -> anyhow::Result<()> {
    // 32 fixed bytes stand in for a deployed program's public address.
    let program = Pubkey::new_from_array([7; 32]);
    let record = 42_u64.to_le_bytes();
    let (address, bump) = derive_record_address(&program, 42)?;
    let verified = Pubkey::create_program_address(&[b"record", &record, &[bump]], &program)?;
    anyhow::ensure!(address == verified, "program address mismatch");
    println!("Solana program address: {address}, bump: {bump}");
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Derives stable, distinct addresses for separate records.
    #[test]
    fn record_ids_select_distinct_addresses() -> anyhow::Result<()> {
        let program = Pubkey::new_from_array([7; 32]);
        let (first, bump) = derive_record_address(&program, 42)?;
        assert_eq!(derive_record_address(&program, 42)?, (first, bump));
        assert_ne!(derive_record_address(&program, 43)?.0, first);
        run()
    }
}
