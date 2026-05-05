#![allow(dead_code)]
// ANCHOR: example
use std::fs::create_dir_all;
use std::path::Path;

use anyhow::Context;
use anyhow::Result;
use glob::MatchOptions;
use glob::glob_with;
use image::ImageFormat;
use image::Rgb;
use image::RgbImage;
use image::imageops::FilterType;
use rayon::prelude::*;

fn main() -> Result<()> {
    // Create test files for the test function.
    create_test_files()?;

    let options: MatchOptions = Default::default();
    let files: Vec<_> = glob_with("temp/test_images/*.jpg", options)?
        .filter_map(|x| x.ok())
        .collect();

    if files.is_empty() {
        println!("No .jpg files found in temp/test_images");
        return Ok(());
    }

    let thumb_dir = "temp/thumbnails";
    create_dir_all(thumb_dir)?;

    println!("Saving {} thumbnails into '{thumb_dir}'...", files.len());

    let image_failures: Vec<_> = files
        .par_iter()
        .map(|path| {
            make_thumbnail(path, thumb_dir, 300).with_context(|| {
                format!("Failed to create thumbnail for: {}", path.display())
            })
        })
        .filter_map(|x| x.err())
        .collect();

    image_failures.iter().for_each(|x| println!("{x}"));

    println!(
        "{} thumbnails saved successfully",
        files.len() - image_failures.len()
    );
    Ok(())
}

fn make_thumbnail<PA, PB>(
    original: PA,
    thumb_dir: PB,
    longest_edge: u32,
) -> Result<()>
where
    PA: AsRef<Path>,
    PB: AsRef<Path>,
{
    let img = image::open(original.as_ref())?;
    let file_path = thumb_dir
        .as_ref()
        .join(original.as_ref().file_name().unwrap());

    Ok(img
        .resize(longest_edge, longest_edge, FilterType::Nearest)
        .save(file_path)?)
}

fn create_test_files() -> Result<()> {
    // Create a directory for test files.
    let test_dir = Path::new("temp/test_images");
    if !test_dir.exists() {
        std::fs::create_dir_all(test_dir)?;
    }

    // Create a few small valid JPEG image files for the example.
    for i in 0..3 {
        let file_path = test_dir.join(format!("test_{i}.jpg"));
        let mut image = RgbImage::new(32, 32);
        for (x, y, pixel) in image.enumerate_pixels_mut() {
            let value = ((x + y + i) % 256) as u8;
            *pixel = Rgb([value, 255 - value, value / 2]);
        }
        image.save_with_format(&file_path, ImageFormat::Jpeg)?;
    }
    Ok(())
}
// ANCHOR_END: example

pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_main() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
// [review; rayon_thumbnails: address the need for test jpg data_parallelism: rayon_thumbnails.rs is noplayground - linking with cc](https://github.com/john-cd/rust_howto/issues/261)
