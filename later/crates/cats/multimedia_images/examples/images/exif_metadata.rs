//! Reads an orientation tag from a generated TIFF/EXIF record.

// ANCHOR: example
use exif::In;
use exif::Reader;
use exif::Tag;

/// Reads EXIF orientation without depending on a camera photograph.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Little-endian TIFF header, one SHORT Orientation field (value 6),
    // and no next IFD. Real JPEG/TIFF files can use read_from_container.
    let tiff = vec![
        b'I', b'I', 42, 0, 8, 0, 0, 0, 1, 0, 0x12, 0x01, 3, 0, 1, 0, 0, 0, 6,
        0, 0, 0, 0, 0, 0, 0,
    ];
    let metadata = Reader::new().read_raw(tiff)?;
    let field = metadata
        .get_field(Tag::Orientation, In::PRIMARY)
        .ok_or("generated EXIF record must contain Orientation")?;
    assert_eq!(field.value.get_uint(0), Some(6));
    println!(
        "Orientation: {}",
        field.display_value().with_unit(&metadata)
    );
    Ok(())
}
// ANCHOR_END: example

/// Runs the in-memory EXIF recipe.
pub fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    main()
}

#[cfg(test)]
mod tests {
    /// Reads a known orientation value from a small generated EXIF record.
    #[test]
    fn reads_generated_orientation()
    -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        super::main()
    }
}
