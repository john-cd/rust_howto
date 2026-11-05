// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A basic one-dimensional rocket launch simulation.
//!
//! Simulations like this one let engineers test a design before building it.
//! The approach is to compute the forces on the vehicle, derive its
//! acceleration (Newton's second law: force = mass x acceleration), and then
//! advance velocity and altitude in small time steps. This simple stepping
//! method is called Euler integration; it is easy to follow, but it is only
//! accurate for small steps.

/// A basic 1D rocket launch simulation.
struct Rocket {
    /// Mass of the rocket without fuel, in kg.
    mass_empty: f64, // kg
    /// Mass of the fuel, in kg.
    fuel_mass: f64, // kg
    /// Thrust of the engine, in N.
    thrust: f64, // N
    /// Fuel burn rate, in kg/s.
    burn_rate: f64, // kg/s
    /// Drag coefficient; dimensionless.
    drag_coeff: f64, // dimensionless
    /// Cross-section area, in m^2.
    area: f64, // m^2
    /// Vertical velocity, in m/s.
    velocity: f64, // m/s
    /// Altitude, in m.
    altitude: f64, // m
}

impl Rocket {
    /// Returns the mass of the rocket with its remaining fuel, in kg.
    fn total_mass(&self) -> f64 {
        self.mass_empty + self.fuel_mass
    }

    /// Advances the simulation by `dt` seconds.
    fn update(&mut self, dt: f64) {
        let g = 9.81; // Gravity (m/s^2)
        // Air gets thinner with height. This exponential model starts from the
        // sea-level density (1.225 kg/m^3) and shrinks by a factor of e every
        // 8 km.
        let air_density = 1.225 * (-self.altitude / 8000.0).exp();

        // Drag is the air resistance that opposes motion; it grows with the
        // square of the speed. `signum` makes it point against the velocity.
        let drag = 0.5
            * air_density
            * self.velocity.powi(2)
            * self.drag_coeff
            * self.area
            * self.velocity.signum();

        // The engine only pushes while fuel remains.
        let current_thrust = if self.fuel_mass > 0.0 {
            self.thrust
        } else {
            0.0
        };

        let mass = self.total_mass();
        // Net force = thrust - drag - weight; dividing by the mass gives the
        // acceleration. The mass shrinks as fuel burns, so it is recomputed
        // on every step.
        let acceleration = (current_thrust - drag - mass * g) / mass;

        self.velocity += acceleration * dt;
        self.altitude += self.velocity * dt;

        if self.fuel_mass > 0.0 {
            self.fuel_mass -= self.burn_rate * dt;
            if self.fuel_mass < 0.0 {
                self.fuel_mass = 0.0;
            }
        }

        // Prevent rocket from going underground
        if self.altitude < 0.0 {
            self.altitude = 0.0;
            self.velocity = 0.0;
        }
    }
}

/// Simulates a launch and prints the flight data.
fn main() {
    let mut rocket = Rocket {
        mass_empty: 500.0,
        fuel_mass: 1500.0,
        thrust: 30000.0,
        burn_rate: 20.0, // Burns for 75 seconds
        drag_coeff: 0.75,
        area: 1.5,
        velocity: 0.0,
        altitude: 0.0,
    };

    // 100 steps of 0.1 s simulate 10 seconds of flight.
    for _ in 0..100 {
        rocket.update(0.1);
    }
    println!("Altitude after 10 seconds: {:.1} m", rocket.altitude);
}
// ANCHOR_END: example

// ANCHOR: sgp4-tle-propagation
/// Propagates a fixed ISS two-line element set sixty minutes from its epoch.
///
/// A two-line element set (TLE) describes a satellite's orbit; SGP4 predicts
/// where the satellite will be. The result is the position in kilometers.
fn propagate_tle_sixty_minutes() -> [f64; 3] {
    use sgp4::Constants;
    use sgp4::Elements;
    use sgp4::MinutesSinceEpoch;

    let elements = Elements::from_tle(
        Some("ISS (ZARYA)".to_owned()),
        b"1 25544U 98067A   26113.55475666  .00008745  00000+0  16716-3 0  9994",
        b"2 25544  51.6320 210.5007 0006819 341.8336  18.2408 15.48911267563287",
    )
    .expect("the fixed ISS TLE should parse");
    let constants = Constants::from_elements(&elements)
        .expect("the fixed ISS TLE should provide valid SGP4 constants");
    let prediction = constants
        .propagate(MinutesSinceEpoch(60.0))
        .expect("the fixed ISS TLE should propagate sixty minutes from epoch");

    prediction.position
}
// ANCHOR_END: sgp4-tle-propagation

// ANCHOR: rk4-ballistic-trajectory
use ode_solvers::Rk4;
use ode_solvers::System;
use ode_solvers::Vector2;

/// Models vertical ballistic motion under constant gravity.
///
/// The state is `[height, velocity]`. A solver repeatedly asks for the
/// derivative of the state: the height changes at the velocity, and the
/// velocity changes at `-g`. RK4 (fourth-order Runge-Kutta) is a more accurate
/// stepping method than the Euler method used above.
struct ConstantGravity;

impl System<f64, Vector2<f64>> for ConstantGravity {
    fn system(&self, _time: f64, y: &Vector2<f64>, dy: &mut Vector2<f64>) {
        dy[0] = y[1];
        dy[1] = -9.81;
    }
}

/// Solves a ten-second vertical trajectory with a fixed-step RK4 integrator.
fn solve_ballistic_trajectory() -> [f64; 2] {
    let initial_state = Vector2::new(0.0, 100.0);
    let mut solver = Rk4::new(ConstantGravity, 0.0, initial_state, 10.0, 0.1);
    solver
        .integrate()
        .expect("the constant-gravity trajectory should integrate");

    let final_state = solver
        .y_out()
        .last()
        .expect("the RK4 solver should return its final state");
    [final_state[0], final_state[1]]
}
// ANCHOR_END: rk4-ballistic-trajectory

/// Runs the aerospace simulation examples through the crate entry point.
pub fn run() {
    main();
    let position = propagate_tle_sixty_minutes();
    let trajectory = solve_ballistic_trajectory();
    println!("ISS position after 60 minutes (km, TEME): {position:?}");
    println!("Ballistic altitude after 10 seconds (m): {}", trajectory[0]);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the aero simulation example through its entry point.
    #[test]
    fn test_aero_simulation_example_entry_point() {
        main();
    }

    /// Checks fuel consumption, ascent, descent, and the ground boundary.
    #[test]
    fn test_rocket_simulation() {
        let mut rocket = Rocket {
            mass_empty: 500.0,
            fuel_mass: 1500.0,
            thrust: 30000.0,
            burn_rate: 20.0,
            drag_coeff: 0.75,
            area: 1.5,
            velocity: 0.0,
            altitude: 0.0,
        };

        let initial_mass = rocket.total_mass();
        rocket.update(0.1);
        assert!(rocket.total_mass() < initial_mass);
        assert!(rocket.altitude > 0.0);

        let mut saw_descent_after_burnout = false;
        let mut landed = false;
        for _ in 0..10_000 {
            let previous_altitude = rocket.altitude;
            rocket.update(0.1);
            assert!(rocket.altitude >= 0.0);
            if rocket.fuel_mass == 0.0 && rocket.altitude < previous_altitude {
                saw_descent_after_burnout = true;
            }
            if saw_descent_after_burnout && rocket.altitude == 0.0 {
                landed = true;
                break;
            }
        }
        assert!(
            saw_descent_after_burnout,
            "rocket should descend after burnout"
        );
        assert!(landed, "rocket should return to the ground");
    }

    /// Checks that a fixed ISS TLE yields a finite position near low Earth
    /// orbit after an hour.
    #[test]
    fn fixed_tle_propagates_to_a_finite_low_earth_orbit_position() {
        let position = propagate_tle_sixty_minutes();
        assert!(position.iter().all(|coordinate| coordinate.is_finite()));

        let radius = position
            .iter()
            .map(|coordinate| coordinate.powi(2))
            .sum::<f64>()
            .sqrt();
        assert!(
            (6_500.0..7_500.0).contains(&radius),
            "expected a low Earth orbit radius in km, got {radius} km"
        );
    }

    /// Checks that RK4 solves the constant-gravity trajectory within tolerance.
    #[test]
    fn rk4_solves_a_constant_gravity_trajectory() {
        let final_state = solve_ballistic_trajectory();
        assert!((final_state[0] - 509.5).abs() < 1e-6);
        assert!((final_state[1] - 1.9).abs() < 1e-6);
    }
}
