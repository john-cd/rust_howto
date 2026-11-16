//! Runs the geo examples.
//!
//! TODO Reviewed ??-??-2026.

/// Measure a unit square and compare two city coordinates with `geo`.
mod geo;
/// Turns an address into coordinates with the `geocoding` crate.
mod geocode;
/// Reads a GeoJSON feature and checks its planar geometry.
mod geojson_polygon;
/// Runs GEOS spatial predicates and operations through the `geos` crate.
mod geos;
/// Computes the great-circle distance between two places with the
mod haversine;
/// Uses an R-tree to find a nearby point without a network lookup.
mod spatial_index;
/// Recognizes US states by name or by abbreviation with `us-state-info`.
mod us_states;

/// Runs the geo examples. The online geocoding query is not run.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut command = clap::Command::new("examples");
    command = command.subcommand(clap::Command::new("geo"));
    command = command.subcommand(clap::Command::new("haversine"));
    command = command.subcommand(clap::Command::new("us-states"));
    command = command.subcommand(clap::Command::new("geocode"));
    command = command.subcommand(clap::Command::new("geojson-polygon"));
    command = command.subcommand(clap::Command::new("geos"));
    command = command.subcommand(clap::Command::new("spatial-index"));
    let matches = command
        .subcommand_required(true)
        .arg_required_else_help(true)
        .get_matches();
    match matches.subcommand() {
        Some(("geo", _)) => {
            geo::run();
            Ok(())
        }
        Some(("haversine", _)) => {
            haversine::run();
            Ok(())
        }
        Some(("us-states", _)) => {
            us_states::run();
            Ok(())
        }
        Some(("geocode", _)) => {
            geocode::run()?;
            Ok(())
        }
        Some(("geojson-polygon", _)) => {
            geojson_polygon::run()?;
            Ok(())
        }
        Some(("geos", _)) => {
            geos::run()?;
            Ok(())
        }
        Some(("spatial-index", _)) => {
            spatial_index::run();
            Ok(())
        }
        _ => unreachable!("Clap requires one of the declared subcommands"),
    }
}
