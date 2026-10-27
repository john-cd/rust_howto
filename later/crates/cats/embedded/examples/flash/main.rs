//! TODO Reviewed ??-??-2026.

// ANCHOR: example
use embedded_storage::nor_flash::ErrorType;
use embedded_storage::nor_flash::NorFlash;
use embedded_storage::nor_flash::NorFlashError;
use embedded_storage::nor_flash::NorFlashErrorKind;
use embedded_storage::nor_flash::ReadNorFlash;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FlashError {
    OutOfBounds,
    Unaligned,
    NeedsErase,
}

impl core::fmt::Display for FlashError {
    /// Formats the simulated NOR flash error for diagnostics.
    fn fmt(
        &self,
        formatter: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        write!(formatter, "NOR flash operation failed: {self:?}")
    }
}

impl std::error::Error for FlashError {}

impl NorFlashError for FlashError {
    /// Maps model-specific errors to the portable NOR flash error category.
    fn kind(&self) -> NorFlashErrorKind {
        NorFlashErrorKind::Other
    }
}

struct RamNorFlash {
    bytes: [u8; 16],
}

impl Default for RamNorFlash {
    /// Creates an erased in-memory flash region.
    fn default() -> Self {
        Self { bytes: [0xff; 16] }
    }
}

impl ErrorType for RamNorFlash {
    type Error = FlashError;
}

impl ReadNorFlash for RamNorFlash {
    const READ_SIZE: usize = 1;

    /// Copies bytes from the in-memory flash region.
    fn read(
        &mut self,
        offset: u32,
        bytes: &mut [u8],
    ) -> Result<(), Self::Error> {
        let start =
            usize::try_from(offset).map_err(|_| FlashError::OutOfBounds)?;
        let end = start
            .checked_add(bytes.len())
            .filter(|end| *end <= self.bytes.len())
            .ok_or(FlashError::OutOfBounds)?;
        bytes.copy_from_slice(&self.bytes[start..end]);
        Ok(())
    }

    /// Returns the capacity of the in-memory flash region in bytes.
    fn capacity(&self) -> usize {
        self.bytes.len()
    }
}

impl NorFlash for RamNorFlash {
    const ERASE_SIZE: usize = 8;
    const WRITE_SIZE: usize = 2;

    /// Programs bytes only when every bit transition is from one to zero.
    fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
        let start =
            usize::try_from(offset).map_err(|_| FlashError::OutOfBounds)?;
        let end = start
            .checked_add(bytes.len())
            .filter(|end| *end <= self.bytes.len())
            .ok_or(FlashError::OutOfBounds)?;
        if !start.is_multiple_of(Self::WRITE_SIZE)
            || !bytes.len().is_multiple_of(Self::WRITE_SIZE)
        {
            return Err(FlashError::Unaligned);
        }
        if self.bytes[start..end]
            .iter()
            .zip(bytes)
            .any(|(old, new)| old & new != *new)
        {
            return Err(FlashError::NeedsErase);
        }
        for (old, new) in self.bytes[start..end].iter_mut().zip(bytes) {
            *old &= new;
        }
        Ok(())
    }

    /// Erases a half-open range whose bounds align to complete erase blocks.
    fn erase(&mut self, from: u32, to: u32) -> Result<(), Self::Error> {
        let start =
            usize::try_from(from).map_err(|_| FlashError::OutOfBounds)?;
        let end = usize::try_from(to).map_err(|_| FlashError::OutOfBounds)?;
        if start > end || end > self.bytes.len() {
            return Err(FlashError::OutOfBounds);
        }
        if !start.is_multiple_of(Self::ERASE_SIZE)
            || !end.is_multiple_of(Self::ERASE_SIZE)
        {
            return Err(FlashError::Unaligned);
        }
        self.bytes[start..end].fill(0xff);
        Ok(())
    }
}

/// Erases, writes, and reads a record through the portable NOR flash traits.
fn main() -> anyhow::Result<()> {
    run_example()
}

/// Stores and reads a record through the in-memory NOR flash model.
fn run_example() -> anyhow::Result<()> {
    let mut flash = RamNorFlash::default();
    flash.erase(0, 8)?;
    flash.write(0, b"rust")?;
    let mut record = [0; 4];
    flash.read(0, &mut record)?;
    println!("stored record: {}", String::from_utf8_lossy(&record));
    Ok(())
}
// ANCHOR_END: example

/// Runs the book example from the example crate's module runner.
pub fn run() -> anyhow::Result<()> {
    run_example()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the example entry point stores and reads its record.
    /// Runs the embedded example through its entry point.
    #[test]
    fn test_embedded_example_entry_point() -> anyhow::Result<()> {
        main()
    }

    /// Verifies NOR flash rejects zero-to-one programming until the block is
    /// erased.
    #[test]
    fn requires_erase_before_zero_to_one_write() {
        let mut flash = RamNorFlash::default();
        flash
            .write(0, b"rust")
            .expect("erased flash accepts writes");
        assert_eq!(flash.write(0, &[0xff; 2]), Err(FlashError::NeedsErase));
        flash
            .erase(0, 8)
            .expect("the range is one aligned erase block");
        flash
            .write(0, b"RUST")
            .expect("erased flash accepts writes");

        let mut record = [0; 4];
        flash.read(0, &mut record).expect("record is in bounds");
        assert_eq!(&record, b"RUST");
    }
}
