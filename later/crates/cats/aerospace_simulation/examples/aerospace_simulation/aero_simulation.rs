#![allow(dead_code)]
// ANCHOR: example
/// A basic 1D rocket launch simulation.
struct Rocket {
    mass_empty: f64,   // kg
    fuel_mass: f64,    // kg
    thrust: f64,       // N
    burn_rate: f64,    // kg/s
    drag_coeff: f64,   // dimensionless
    area: f64,         // m^2
    velocity: f64,     // m/s
    altitude: f64,     // m
}

impl Rocket {
    fn total_mass(&self) -> f64 {
        self.mass_empty + self.fuel_mass
    }

    fn update(&mut self, dt: f64) {
        let g = 9.81; // Gravity (m/s^2)
        // Exponential atmosphere model
        let air_density = 1.225 * (-self.altitude / 8000.0).exp();

        let drag = 0.5 * air_density * self.velocity.powi(2) * self.drag_coeff * self.area * self.velocity.signum();

        let current_thrust = if self.fuel_mass > 0.0 {
            self.thrust
        } else {
            0.0
        };

        let mass = self.total_mass();
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

pub fn main() {
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

    let dt = 0.1;
    let mut time = 0.0;

    let mut final_altitude = 0.0;

    while time <= 10.0 {
        rocket.update(dt);
        time += dt;
        final_altitude = rocket.altitude;
    }

    assert!(final_altitude > 0.0);
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rocket_simulation() {
        main();

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
        rocket.update(1.0); // 1 second update

        // The rocket should move upwards and burn fuel
        assert!(rocket.altitude > 0.0, "Rocket should have positive altitude");
        assert!(rocket.velocity > 0.0, "Rocket should have positive velocity");
        assert!(rocket.fuel_mass < 1500.0, "Rocket should have burned some fuel");
        assert!(rocket.total_mass() < initial_mass, "Total mass should decrease");
    }
}
