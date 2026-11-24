// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates reading from a memory-mapped file.
//!
//! A file-backed mapping exposes file contents as bytes that the operating
//! system can page in on demand. The caller must prevent changes to the file
//! that would invalidate reads through the mapping.

use std::io::Error;
use std::io::Write as _;

use memmap2::Mmap;
use memmap2::MmapMut;
use memmap2::MmapOptions;

/// Writes a file, then reads it through a read-only memory map.
fn main() -> Result<(), Error> {
    let mut file = tempfile::tempfile()?;
    write!(file, "My hovercraft is full of eels!")?;
    // SAFETY: Each invocation owns a fresh file. No handle escapes and no
    // writes occur while the mapping is live.
    let map = unsafe { Mmap::map(&file)? };
    // `Mmap` is `Deref<Target = [u8]>` and therefore can be used like a slice.
    // Assert that a slice of the memory map matches the expected bytes:
    assert_eq!(&map[3..13], b"hovercraft");
    // Read bytes at selected offsets in the mapping.
    let random_indexes = [0, 1, 2, 19, 22, 10, 11, 29];
    let random_bytes: Vec<u8> = random_indexes.iter().map(|&idx| map[idx]).collect();
    assert_eq!(&*random_bytes, b"My loaf!");
    // A file-backed memory map remains valid even after the File is dropped.

    // You can also create mutable memory-mapped buffers - either file-backed or
    // anonymous - with `MmapMut`.
    // - A file-backed buffer may be used to read from or write to a file.
    // - An anonymous buffer may be used any place that an in-memory byte buffer is needed.
    let mut mmap_mut: MmapMut = MmapOptions::new().len(13).map_anon()?;
    mmap_mut.copy_from_slice(b"Hello, world!");

    Ok(())
}
// ANCHOR_END: example

/// Writes a file, then reads it through a read-only memory map.
///
/// # Errors
///
/// Returns an error if a step of the example fails.
pub fn run() -> Result<(), Error> {
    main()
}

/// Verifies that the memmap2 example's main path succeeds.
#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the memmap2 example's main path succeeds.
    #[test]
    fn test_memmap2_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
