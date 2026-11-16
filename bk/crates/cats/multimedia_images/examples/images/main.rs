//! Runs the images examples.
//!
//! TODO Reviewed ??-??-2026.

use std::path::PathBuf;

/// Blur an image in memory using a Gaussian filter.
mod blur;
/// Encode an image to PNG and JPEG in memory.
mod convert;
/// Reads an orientation tag from a generated TIFF/EXIF record.
mod exif_metadata;
/// Generate and save a simple RGB image using the `image` crate.
mod images;
/// Optimizes a generated PNG and verifies its decoded pixels.
mod optimize_png;
/// Create an image in memory, resize it and convert it to grayscale.
mod resize;

/// Runs the images examples.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut command = clap::Command::new("examples");
    command = command.subcommand(clap::Command::new("blur"));
    command = command.subcommand(clap::Command::new("convert"));
    command = command.subcommand(clap::Command::new("exif-metadata"));
    command = command.subcommand(clap::Command::new("optimize-png"));
    command = command.subcommand(clap::Command::new("resize"));
    command = command.subcommand(
        clap::Command::new("images").arg(
            clap::Arg::new("output")
                .value_parser(clap::value_parser!(PathBuf))
                .help("Output path (defaults to generated_image.png)"),
        ),
    );
    let matches = command
        .subcommand_required(true)
        .arg_required_else_help(true)
        .get_matches();
    match matches.subcommand() {
        Some(("blur", _)) => {
            blur::run();
            Ok(())
        }
        Some(("convert", _)) => {
            convert::run()?;
            Ok(())
        }
        Some(("exif-metadata", _)) => {
            exif_metadata::run()?;
            Ok(())
        }
        Some(("optimize-png", _)) => {
            optimize_png::run()?;
            Ok(())
        }
        Some(("resize", _)) => {
            resize::run();
            Ok(())
        }
        Some(("images", args)) => {
            images::run_with_output_path(args.get_one::<PathBuf>("output").cloned())?;
            Ok(())
        }
        _ => unreachable!("Clap requires one of the declared subcommands"),
    }
}
