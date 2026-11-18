//! Runs the SVG parsing and raster-rendering examples.
//!
//! TODO Reviewed ??-??-2026.

/// Parses an SVG and rasterizes it to a PNG with `resvg`.
mod resvg;
/// Parses and inspects the normalized SVG tree produced by `usvg`.
mod usvg;

/// Runs the two SVG examples.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut command = clap::Command::new("examples");
    command = command.subcommand(clap::Command::new("resvg"));
    command = command.subcommand(clap::Command::new("usvg"));
    let matches = command
        .subcommand_required(true)
        .arg_required_else_help(true)
        .get_matches();
    match matches.subcommand() {
        Some(("resvg", _)) => {
            resvg::run()?;
            Ok(())
        }
        Some(("usvg", _)) => {
            usvg::run()?;
            Ok(())
        }
        _ => unreachable!("Clap requires one of the declared subcommands"),
    }
}
