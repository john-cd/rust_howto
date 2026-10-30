// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Drive a simulated first-order plant to a setpoint with a PI controller.
//!
//! A PID controller keeps a measured value, such as a motor speed, a drone's
//! altitude or an oven temperature, close to a target (the setpoint). Each
//! cycle it compares the measurement to the target and computes a control
//! output from three terms: the proportional term reacts to the current error,
//! the integral term accumulates past error to remove a lasting offset, and
//! the derivative term reacts to how fast the error is changing (unused here,
//! which makes this a PI controller). In robotics, "plant" means the system
//! being controlled.

use pid::Pid;

/// A first-order plant, such as a motor speed or a heater temperature:
/// `tau * dy/dt = -y + u`.
struct Plant {
    output: f32,
    tau: f32,
}

impl Plant {
    /// Advances the simulation by `dt` seconds.
    fn step(&mut self, input: f32, dt: f32) -> f32 {
        self.output += dt * (input - self.output) / self.tau;
        self.output
    }
}

/// Controls the plant for 20 simulated seconds and returns the final error.
fn simulate() -> f32 {
    let setpoint = 10.0_f32;
    // `Pid::new(setpoint, output_limit)`; each term has its own limit.
    let mut controller = Pid::new(setpoint, 100.0_f32);
    controller.p(2.0_f32, 100.0_f32).i(1.0_f32, 50.0_f32);

    let mut plant = Plant {
        output: 0.0,
        tau: 1.0,
    };
    // Control loop: measure, compute the output, apply it, repeat. 200 steps
    // of 0.1 s simulate 20 seconds.
    let dt = 0.1;
    let mut measurement = plant.output;
    for _ in 0..200 {
        let control = controller.next_control_output(measurement);
        measurement = plant.step(control.output, dt);
    }
    (setpoint - measurement).abs()
}

/// Prints the remaining error.
fn main() {
    let error = simulate();
    println!("Final error: {error}");
    assert!(error < 0.05);
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the pid example through its entry point.
    #[test]
    fn test_pid_example_entry_point() {
        main();
    }

    #[test]
    fn controller_tracks_the_setpoint() {
        assert!(simulate() < 0.05);
    }

    #[test]
    fn plant_is_a_first_order_lag() {
        let mut plant = Plant {
            output: 0.0,
            tau: 1.0,
        };
        let first = plant.step(1.0, 0.1);
        assert!((first - 0.1).abs() < 1e-6);
        let second = plant.step(1.0, 0.1);
        assert!(second > first && second < 1.0);
    }
}
