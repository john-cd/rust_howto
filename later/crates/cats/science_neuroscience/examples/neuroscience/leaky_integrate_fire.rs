// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Example: simulate a leaky integrate-and-fire neuron with constant current.
//!
//! Euler integration advances the membrane potential in one-millisecond steps.
//! The example prints the deterministic spike times for a suprathreshold input.

const RESTING_POTENTIAL_MV: f64 = -65.0;
const THRESHOLD_POTENTIAL_MV: f64 = -50.0;
const RESET_POTENTIAL_MV: f64 = RESTING_POTENTIAL_MV;
const MEMBRANE_RESISTANCE_MOHM: f64 = 15.0;
const MEMBRANE_TIME_CONSTANT_MS: f64 = 10.0;
const TIME_STEP_MS: f64 = 1.0;

/// Simulates a leaky integrate-and-fire neuron and returns spike times in
/// milliseconds.
fn simulate_spike_times(input_current_na: f64, duration_ms: u32) -> Vec<u32> {
    let mut membrane_potential_mv = RESTING_POTENTIAL_MV;
    let mut spike_times_ms = Vec::new();

    for elapsed_ms in 1..=duration_ms {
        let voltage_change_mv = TIME_STEP_MS / MEMBRANE_TIME_CONSTANT_MS
            * (RESTING_POTENTIAL_MV - membrane_potential_mv
                + MEMBRANE_RESISTANCE_MOHM * input_current_na);
        membrane_potential_mv += voltage_change_mv;

        if membrane_potential_mv >= THRESHOLD_POTENTIAL_MV {
            spike_times_ms.push(elapsed_ms);
            membrane_potential_mv = RESET_POTENTIAL_MV;
        }
    }

    spike_times_ms
}

/// Runs the simulation and prints spike times for a suprathreshold input.
fn main() {
    let spike_times_ms = simulate_spike_times(2.0, 100);
    println!(
        "Neuron fired {} spikes at {spike_times_ms:?} ms",
        spike_times_ms.len()
    );
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::simulate_spike_times;

    /// Verifies that a fixed suprathreshold current produces a repeatable spike
    /// count.
    #[test]
    fn suprathreshold_current_produces_fourteen_spikes() {
        assert_eq!(simulate_spike_times(2.0, 100).len(), 14);
    }

    /// Verifies that a subthreshold steady-state voltage does not produce
    /// spikes.
    #[test]
    fn subthreshold_current_produces_no_spikes() {
        assert!(simulate_spike_times(0.5, 100).is_empty());
    }

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
