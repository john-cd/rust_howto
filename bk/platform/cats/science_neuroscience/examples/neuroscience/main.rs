//! Runs the neuroscience example.
//!
//! TODO Reviewed ??-??-2026.

/// Analyze a synthetic sampled signal with a fast Fourier transform.
mod fft_signal;
/// Example: simulate a leaky integrate-and-fire neuron with constant current.
mod leaky_integrate_fire;
/// Example: create a dummy neuroimaging volume and save it as a `NIfTI` file.
mod neuro;

/// Runs the neuroscience example.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut command = clap::Command::new("examples");
    command = command.subcommand(clap::Command::new("neuro"));
    command = command.subcommand(clap::Command::new("leaky-integrate-fire"));
    command = command.subcommand(clap::Command::new("fft-signal"));
    let matches = command
        .subcommand_required(true)
        .arg_required_else_help(true)
        .get_matches();
    match matches.subcommand() {
        Some(("neuro", _)) => {
            neuro::run()?;
            Ok(())
        }
        Some(("leaky-integrate-fire", _)) => {
            leaky_integrate_fire::run();
            Ok(())
        }
        Some(("fft-signal", _)) => {
            fft_signal::run();
            Ok(())
        }
        _ => unreachable!("Clap requires one of the declared subcommands"),
    }
}
