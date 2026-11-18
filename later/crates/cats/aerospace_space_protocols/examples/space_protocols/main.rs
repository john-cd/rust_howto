//! Runs the space protocols example.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

/// Create and parse a PUS-C telecommand.
mod pus_telecommand;
/// Create and read a PUS-C connection-test telemetry packet.
mod pus_telemetry;
/// Build and parse CCSDS space packets and ECSS PUS packets.
mod space_protocols;

/// Selects a packet demonstration.
#[derive(Parser)]
struct Cli {
    /// Runs all packet examples when omitted.
    #[command(subcommand)]
    recipe: Option<Recipe>,
}

/// Packet demonstrations available from the command line.
#[derive(Subcommand)]
enum Recipe {
    /// Constructs and checks all three packet types.
    All,
    /// Constructs and checks a CCSDS space packet.
    Ccsds,
    /// Constructs and checks a PUS-C telecommand.
    Telecommand,
    /// Constructs and checks PUS-C telemetry.
    Telemetry,
}

/// Runs the selected packet demonstration.
fn main() {
    match Cli::parse().recipe.unwrap_or(Recipe::All) {
        Recipe::All => {
            space_protocols::run();
            pus_telecommand::run();
            pus_telemetry::run();
        }
        Recipe::Ccsds => space_protocols::run(),
        Recipe::Telecommand => pus_telecommand::run(),
        Recipe::Telemetry => pus_telemetry::run(),
    }
}
