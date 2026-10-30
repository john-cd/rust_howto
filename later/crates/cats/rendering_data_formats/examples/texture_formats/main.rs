//! Round-trips DDS and KTX2 container metadata without a GPU.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: example
use ddsfile::AlphaMode;
use ddsfile::D3D10ResourceDimension;
use ddsfile::Dds;
use ddsfile::DxgiFormat;
use ddsfile::NewDxgiParams;
use ktx2::Format;
use ktx2::Header;
use ktx2::Index;

/// Writes a DDS container with room for a mip chain, then reads it back.
///
/// `ddsfile` handles the container: the pixel data stays zeroed and
/// undecoded.
fn dds_round_trip() -> anyhow::Result<(u32, u32, u32, Option<DxgiFormat>)> {
    let dds = Dds::new_dxgi(NewDxgiParams {
        height: 64,
        width: 64,
        depth: None,
        format: DxgiFormat::BC7_UNorm,
        mipmap_levels: Some(7),
        array_layers: None,
        caps2: None,
        is_cubemap: false,
        resource_dimension: D3D10ResourceDimension::Texture2D,
        alpha_mode: AlphaMode::Unknown,
    })?;

    let mut bytes = Vec::new();
    dds.write(&mut bytes)?;
    let parsed = Dds::read(bytes.as_slice())?;
    Ok((
        parsed.get_width(),
        parsed.get_height(),
        parsed.get_num_mipmap_levels(),
        parsed.get_dxgi_format(),
    ))
}

/// Encodes a KTX2 header for a BC7 texture and parses it again.
fn ktx2_header_round_trip() -> anyhow::Result<Header> {
    let header = Header {
        format: Some(Format::BC7_UNORM_BLOCK),
        type_size: 1, // Required for block-compressed formats.
        pixel_width: 64,
        pixel_height: 64,
        pixel_depth: 0,
        layer_count: 0,
        face_count: 1,
        level_count: 7,
        supercompression_scheme: None,
        index: Index {
            dfd_byte_offset: 0,
            dfd_byte_length: 0,
            kvd_byte_offset: 0,
            kvd_byte_length: 0,
            sgd_byte_offset: 0,
            sgd_byte_length: 0,
        },
    };
    Ok(Header::from_bytes(&header.as_bytes())?)
}

/// Prints the metadata that each container reports.
fn main() -> anyhow::Result<()> {
    let (width, height, levels, format) = dds_round_trip()?;
    println!("DDS: {width}x{height}, {levels} levels, {format:?}");
    let header = ktx2_header_round_trip()?;
    println!(
        "KTX2: {}x{}, {} levels, {:?}",
        header.pixel_width,
        header.pixel_height,
        header.level_count,
        header.format
    );
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point.
    #[test]
    fn test_main_entry_point() -> anyhow::Result<()> {
        main()
    }

    /// Verifies the DDS metadata survives a write and read.
    #[test]
    fn dds_metadata_round_trips() -> anyhow::Result<()> {
        assert_eq!(dds_round_trip()?, (64, 64, 7, Some(DxgiFormat::BC7_UNorm)));
        Ok(())
    }

    /// Verifies the KTX2 header survives encoding and parsing.
    #[test]
    fn ktx2_header_round_trips() -> anyhow::Result<()> {
        let header = ktx2_header_round_trip()?;
        assert_eq!(header.format, Some(Format::BC7_UNORM_BLOCK));
        assert_eq!((header.pixel_width, header.level_count), (64, 7));
        Ok(())
    }

    /// Verifies that bytes without the KTX2 magic number are rejected.
    #[test]
    fn ktx2_rejects_bad_magic() {
        assert!(Header::from_bytes(&[0; Header::LENGTH]).is_err());
    }
}
