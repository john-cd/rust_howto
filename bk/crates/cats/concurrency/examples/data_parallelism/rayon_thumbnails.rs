#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Generating thumbnails of images in parallel with `rayon` and `image`.

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context as _;
use anyhow::Result;
use anyhow::anyhow;
use glob::MatchOptions;
use glob::glob_with;
use image::imageops::FilterType;
use rayon::prelude::*;

/// Maximum width or height of the example thumbnails.
const THUMBNAIL_EDGE: u32 = 300;

/// Creates thumbnails for the JPEG files in `input_dir` in parallel.
fn main() -> Result<()> {
    let input_dir = Path::new("images");
    let output_dir = Path::new("temp/thumbnails");
    let thumbnail_count =
        create_thumbnails(input_dir, output_dir, THUMBNAIL_EDGE)?;
    println!(
        "Saved {thumbnail_count} thumbnails into '{}'.",
        output_dir.display()
    );
    Ok(())
}

/// Finds JPEG files in `input_dir` and creates their thumbnails in parallel.
///
/// # Errors
///
/// Returns an error if the glob cannot be read or any image cannot be decoded
/// or written.
fn create_thumbnails(
    input_dir: &Path,
    output_dir: &Path,
    longest_edge: u32,
) -> Result<usize> {
    let pattern = input_dir.join("*.jpg").to_string_lossy().replace('\\', "/");
    let files: Vec<PathBuf> = glob_with(&pattern, MatchOptions::default())?
        .collect::<std::result::Result<_, _>>()?;

    if files.is_empty() {
        return Err(anyhow!("no JPEG files found in {}", input_dir.display()));
    }

    fs::create_dir_all(output_dir)?;
    files.par_iter().try_for_each(|path| {
        make_thumbnail(path, output_dir, longest_edge).with_context(|| {
            format!("creating a thumbnail for {}", path.display())
        })
    })?;

    Ok(files.len())
}

/// Resizes one image and saves it under the same file name in `output_dir`.
fn make_thumbnail(
    original: &Path,
    output_dir: &Path,
    longest_edge: u32,
) -> Result<()> {
    let image = image::open(original)?;
    let file_name = original.file_name().ok_or_else(|| {
        anyhow!("image path has no file name: {}", original.display())
    })?;
    let output_path = output_dir.join(file_name);

    image
        .resize(longest_edge, longest_edge, FilterType::Nearest)
        .save(output_path)?;
    Ok(())
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use image::GenericImageView;
    use image::Rgb;
    use image::RgbImage;

    use super::*;

    /// Creates sample JPEGs in a temporary directory and verifies each output.
    #[test]
    fn generated_jpegs_receive_parallel_thumbnails() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let input_dir = temp_dir.path().join("images");
        let output_dir = temp_dir.path().join("thumbnails");
        fs::create_dir_all(&input_dir)?;

        for index in 0..3 {
            let image = RgbImage::from_pixel(32, 16, Rgb([20, 40, 60]));
            image.save(input_dir.join(format!("image_{index}.jpg")))?;
        }

        let thumbnail_count = create_thumbnails(&input_dir, &output_dir, 16)?;

        assert_eq!(thumbnail_count, 3);
        for index in 0..3 {
            let thumbnail =
                image::open(output_dir.join(format!("image_{index}.jpg")))?;
            assert_eq!(thumbnail.dimensions(), (16, 8));
        }
        Ok(())
    }
}
// [review; rayon_thumbnails: address the need for test jpg data_parallelism: rayon_thumbnails.rs is noplayground - linking with cc](https://github.com/john-cd/rust_howto/issues/261)
