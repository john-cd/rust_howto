// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Fits a viewport and removes padded rows from bounded RGBA GPU readback.

use anyhow::Result;
use anyhow::ensure;

/// A byte layout for a single-layer RGBA8 texture copied into a staging buffer.
#[derive(Debug)]
struct CopyLayout {
    /// Number of bytes of actual RGBA8 pixels in one row.
    row_bytes: usize,
    /// Row stride in the staging buffer, rounded up to the GPU copy alignment.
    padded_row_bytes: usize,
    /// Number of texture rows to unpack.
    height: usize,
    /// Total mapped byte count, including row padding.
    buffer_bytes: usize,
}

impl CopyLayout {
    /// Computes checked row padding with a 64 MiB demonstration allocation limit.
    fn new(width: u32, height: u32) -> Result<Self> {
        ensure!(
            width > 0 && height > 0,
            "texture dimensions must be nonzero"
        );
        let row_bytes = width
            .checked_mul(4)
            .ok_or_else(|| anyhow::anyhow!("row size overflow"))?;
        let alignment = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
        let padded = row_bytes
            .checked_add(alignment - 1)
            .map(|n| n / alignment * alignment)
            .ok_or_else(|| anyhow::anyhow!("row alignment overflow"))?;
        let buffer_bytes = u64::from(padded) * u64::from(height);
        ensure!(
            buffer_bytes <= 64 * 1024 * 1024,
            "readback exceeds demonstration memory budget"
        );
        Ok(Self {
            row_bytes: row_bytes as usize,
            padded_row_bytes: padded as usize,
            height: height as usize,
            buffer_bytes: buffer_bytes as usize,
        })
    }

    /// Removes staging-row padding after mapping has successfully completed.
    fn unpack(&self, mapped: &[u8]) -> Result<Vec<u8>> {
        ensure!(
            mapped.len() == self.buffer_bytes,
            "mapped buffer size mismatch"
        );
        let mut pixels = Vec::with_capacity(self.row_bytes * self.height);
        for row in mapped.chunks_exact(self.padded_row_bytes) {
            pixels.extend_from_slice(&row[..self.row_bytes]);
        }
        Ok(pixels)
    }
}

/// Fits source pixels inside a physical-pixel target without stretching them.
fn fit_viewport(source: [u32; 2], target: [u32; 2]) -> Result<[f64; 4]> {
    ensure!(
        source.into_iter().chain(target).all(|n| n > 0),
        "zero-sized viewport"
    );
    let scale = (f64::from(target[0]) / f64::from(source[0]))
        .min(f64::from(target[1]) / f64::from(source[1]));
    let width = f64::from(source[0]) * scale;
    let height = f64::from(source[1]) * scale;
    Ok([
        (f64::from(target[0]) - width) / 2.0,
        (f64::from(target[1]) - height) / 2.0,
        width,
        height,
    ])
}

/// Maps physical-pixel input to source pixels, rejecting bars and the far edges.
fn image_point(source: [u32; 2], target: [u32; 2], point: [f64; 2]) -> Result<Option<[f64; 2]>> {
    ensure!(
        point.iter().all(|v| v.is_finite()),
        "non-finite pointer position"
    );
    let [x, y, width, height] = fit_viewport(source, target)?;
    if point[0] < x || point[1] < y || point[0] >= x + width || point[1] >= y + height {
        return Ok(None);
    }
    Ok(Some([
        (point[0] - x) / width * f64::from(source[0]),
        (point[1] - y) / height * f64::from(source[1]),
    ]))
}

/// Demonstrates a non-aligned readback row and a letterboxed viewport.
fn main() -> Result<()> {
    let layout = CopyLayout::new(65, 2)?;
    let mapped = vec![0_u8; layout.buffer_bytes];
    println!(
        "Image point: {:?}",
        image_point([1920, 1080], [800, 600], [400.0, 300.0])?
    );
    println!(
        "Packed bytes: {}; viewport: {:?}",
        layout.unpack(&mapped)?.len(),
        fit_viewport([1920, 1080], [800, 600])?
    );
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Padding sentinel bytes never enter the tightly packed image.
    #[test]
    fn padded_rows_are_unpacked() -> Result<()> {
        let layout = CopyLayout::new(65, 2)?;
        assert_eq!(layout.padded_row_bytes, 512);
        let mut mapped = vec![255; layout.buffer_bytes];
        mapped[..260].fill(1);
        mapped[512..772].fill(2);
        let pixels = layout.unpack(&mapped)?;
        assert_eq!(pixels.len(), 520);
        assert!(pixels[..260].iter().all(|v| *v == 1));
        assert!(pixels[260..].iter().all(|v| *v == 2));
        Ok(())
    }

    /// A naturally aligned row needs no additional staging bytes.
    #[test]
    fn aligned_row_is_unchanged() -> Result<()> {
        let layout = CopyLayout::new(64, 1)?;
        assert_eq!(layout.row_bytes, layout.padded_row_bytes);
        assert_eq!(layout.unpack(&vec![7; 256])?, vec![7; 256]);
        Ok(())
    }

    /// Zero, overflowing, oversized, and truncated buffers fail before use.
    #[test]
    fn malformed_layout_is_rejected() -> Result<()> {
        assert!(CopyLayout::new(0, 1).is_err());
        assert!(CopyLayout::new(u32::MAX, 1).is_err());
        assert!(CopyLayout::new(16_384, 16_384).is_err());
        assert!(CopyLayout::new(65, 2)?.unpack(&[0; 4]).is_err());
        assert!(fit_viewport([0, 10], [100, 100]).is_err());
        Ok(())
    }

    /// Pointer coordinates map consistently while letterbox bars and far edges stay outside.
    #[test]
    fn pointer_mapping_matches_viewport() -> Result<()> {
        assert_eq!(
            image_point([1920, 1080], [800, 600], [400.0, 300.0])?,
            Some([960.0, 540.0])
        );
        assert_eq!(image_point([1920, 1080], [800, 600], [400.0, 74.0])?, None);
        assert_eq!(image_point([1920, 1080], [800, 600], [800.0, 300.0])?, None);
        assert!(image_point([1920, 1080], [800, 600], [f64::NAN, 0.0]).is_err());
        Ok(())
    }

    /// Wide and tall sources remain centered and preserve their aspect ratios.
    #[test]
    fn viewport_preserves_aspect() -> Result<()> {
        assert_eq!(
            fit_viewport([1920, 1080], [800, 600])?,
            [0.0, 75.0, 800.0, 450.0]
        );
        assert_eq!(
            fit_viewport([100, 200], [800, 600])?,
            [250.0, 0.0, 300.0, 600.0]
        );
        Ok(())
    }
}
