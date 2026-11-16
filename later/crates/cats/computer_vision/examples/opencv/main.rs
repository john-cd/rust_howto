//! Runs the OpenCV example on Linux when the `opencv` feature is enabled.
//!
//! TODO Reviewed ??-??-2026.

/// Loads a Haar cascade and applies it to a generated grayscale frame.
#[cfg(all(target_os = "linux", feature = "opencv"))]
mod faces;

/// This example demonstrates how to load an image, convert it to grayscale, and
#[cfg(all(target_os = "linux", feature = "opencv"))]
mod opencv;

/// Runs the example if it is supported.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(all(target_os = "linux", feature = "opencv"))]
    {
        let matches = clap::Command::new("examples")
            .subcommand(clap::Command::new("opencv"))
            .subcommand(clap::Command::new("faces"))
            .subcommand_required(true)
            .arg_required_else_help(true)
            .get_matches();
        match matches.subcommand() {
            Some(("opencv", _)) => opencv::run()?,
            Some(("faces", _)) => faces::run()?,
            _ => unreachable!("Clap requires one of the declared subcommands"),
        }
        Ok(())
    }
    #[cfg(not(all(target_os = "linux", feature = "opencv")))]
    {
        println!("OpenCV examples require Linux and the `opencv` feature.");
        Ok(())
    }
}
