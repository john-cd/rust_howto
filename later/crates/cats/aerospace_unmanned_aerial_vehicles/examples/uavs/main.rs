//! Runs the UAV example.
//!
//! TODO Reviewed ??-??-2026.

/// Receiving telemetry from a Tello drone.
mod uavs;

/// Runs the uavs example.
fn main() -> anyhow::Result<()> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    match args.as_slice() {
        [] => uavs::run(),
        [flag] if flag == "--live" => {
            uavs::run_live();
            Ok(())
        }
        _ => anyhow::bail!("usage: uavs [--live]"),
    }
}
