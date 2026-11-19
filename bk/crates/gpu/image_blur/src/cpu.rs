//! Computes the integer blur using two passes over contiguous image rows.

use rayon::prelude::*;

/// Applies the weights of a five-tap Gaussian approximation.
fn weighted(a: u32, b: u32, c: u32, d: u32, e: u32) -> u32 {
    a + 4 * b + 6 * c + 4 * d + e
}

/// Blurs a square 8-bit grayscale image stored in `u32` elements.
///
/// `width` is positive. Each slice has `width * width` elements. The caller
/// supplies scratch and output buffers so repeated samples can reuse allocations.
pub fn blur(width: usize, input: &[u32], scratch: &mut [u32], output: &mut [u32]) {
    scratch
        .par_chunks_mut(width)
        .with_min_len(16)
        .zip(input.par_chunks(width))
        .for_each(|(target, source)| {
            let last = width - 1;
            for (x, pixel) in target.iter_mut().enumerate() {
                *pixel = weighted(
                    source[x.saturating_sub(2)],
                    source[x.saturating_sub(1)],
                    source[x],
                    source[(x + 1).min(last)],
                    source[(x + 2).min(last)],
                );
            }
        });

    output
        .par_chunks_mut(width)
        .with_min_len(16)
        .enumerate()
        .for_each(|(y, target)| {
            let last = width - 1;
            let a = y.saturating_sub(2) * width;
            let b = y.saturating_sub(1) * width;
            let c = y * width;
            let d = (y + 1).min(last) * width;
            let e = (y + 2).min(last) * width;
            for (x, pixel) in target.iter_mut().enumerate() {
                let sum = weighted(
                    scratch[a + x],
                    scratch[b + x],
                    scratch[c + x],
                    scratch[d + x],
                    scratch[e + x],
                );
                *pixel = (sum + 128) / 256;
            }
        });
}

#[cfg(test)]
mod tests {
    use super::blur;

    /// Clamped borders preserve a constant image at every pixel.
    #[test]
    fn constant_image_is_preserved() {
        let mut output = [0; 9];
        blur(3, &[127; 9], &mut [0; 9], &mut output);
        assert_eq!(output, [127; 9]);
    }

    /// A central white pixel spreads with the product of both filter weights.
    #[test]
    fn impulse_matches_gaussian_weights() {
        let mut input = [0; 25];
        input[12] = 255;
        let mut output = [0; 25];
        blur(5, &input, &mut [0; 25], &mut output);
        assert_eq!(
            output,
            [
                1, 4, 6, 4, 1, 4, 16, 24, 16, 4, 6, 24, 36, 24, 6, 4, 16, 24, 16, 4, 1, 4, 6, 4, 1,
            ]
        );
    }
}
