<!-- TODO Reviewed ??-??-2026 -->

# Science Robotics

This crate contains code examples for the **Science Robotics** category on crates.io. It demonstrates integration with key ecosystem crates including `anyhow`, `async-trait`, `bonsai-bt`, `clap`, `futures`, `pathfinding`.

## Examples Covered

- **Artificial Intelligence** (`examples/artificial_intelligence/`): Demonstrates concepts like `astar`, `bonsai_bt`.
- **Control Systems** (`examples/control_systems/`): Demonstrates concepts like `pid`.
- **Hardware Integration** (`examples/hardware_integration/`): Code examples covering hardware integration.
- **Perception and Sensors** (`examples/perception_and_sensors/`): Code examples covering perception and sensors.
- **Robot Operating Systems** (`examples/robot_operating_systems/`): Demonstrates concepts like `ros2_pubsub`.
- **Robotics Frameworks** (`examples/robotics_frameworks/`): Demonstrates concepts like `openrr`, `zenoh`.
- **Simulation Visualization** (`examples/simulation_visualization/`): Demonstrates concepts like `robot_simulation`.

## Usage

To check or run examples in this crate:

```console
cargo check --package science_robotics --examples
cargo test --package science_robotics
```

Code examples correspond to chapters in `bk/src/categories/science_robotics` or related book sections.
