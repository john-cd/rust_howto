//! Runs the visualization example.
//!
//! TODO Reviewed ??-??-2026.

/// This example demonstrates how to create a simple scatter plot using the
mod plotly;
/// Demonstrates rendering a 2D line plot into an SVG string using `plotters`.
mod plotters;

/// Runs the visualization example.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut command = clap::Command::new("examples");
    command = command.subcommand(clap::Command::new("plotly"));
    command = command.subcommand(clap::Command::new("plotters"));
    #[cfg(feature = "chartistry")]
    let command = command.subcommand(
        clap::Command::new("leptos-chartistry")
            .about("Show how to start the Chartistry browser example"),
    );
    let matches = command
        .subcommand_required(true)
        .arg_required_else_help(true)
        .get_matches();
    match matches.subcommand() {
        Some(("plotly", _)) => {
            plotly::run()?;
            Ok(())
        }
        Some(("plotters", _)) => {
            plotters::run()?;
            Ok(())
        }
        #[cfg(feature = "chartistry")]
        Some(("leptos-chartistry", _)) => {
            println!(
                "From bk/crates/cats/visualization, run: trunk serve examples/chartistry/index.html --locked"
            );
            Ok(())
        }
        _ => unreachable!("Clap requires one of the declared subcommands"),
    }
}
