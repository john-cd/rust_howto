//! Encodes and decodes a small AVIF image entirely in memory.

// ANCHOR: example
use avif_decode::Decoder;
use avif_decode::Image;
use imgref::Img;
use ravif::BitDepth;
use ravif::Encoder;
use rgb::RGBA8;

/// Encodes an RGBA buffer and verifies the dimensions after AVIF decoding.
///
/// # Errors
///
/// Returns an error if the pixels cannot be encoded or decoded.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    const WIDTH: usize = 4;
    const HEIGHT: usize = 2;
    let pixels =
        [RGBA8::new(255, 0, 0, 128), RGBA8::new(0, 128, 255, 255)].repeat(WIDTH * HEIGHT / 2);
    let image = Img::new(pixels.as_slice(), WIDTH, HEIGHT);

    let encoded = Encoder::new()
        .with_bit_depth(BitDepth::Eight)
        .with_speed(10)
        .with_num_threads(Some(1))
        .encode_rgba(image)?;

    let decoded = Decoder::from_avif(&encoded.avif_file)?.to_image()?;
    let Image::Rgba8(decoded) = decoded else {
        return Err("expected 8-bit RGBA pixels".into());
    };
    assert_eq!((decoded.width(), decoded.height()), (WIDTH, HEIGHT));
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    /// Checks that encoding and decoding preserve the dimensions of an RGBA image.
    #[test]
    fn round_trips_dimensions() -> Result<(), Box<dyn std::error::Error>> {
        super::main()
    }
}
