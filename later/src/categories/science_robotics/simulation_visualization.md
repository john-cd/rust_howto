<!-- TODO Reviewed ??-??-2026 -->

# Simulation and Visualization

{{#include simulation_visualization.incl.md}}

Robotics simulations combine a physical model, sensor or controller data, and tools for inspection. Keep the simulator headless for repeatable runs, then send recorded state to a viewer or charting backend.

## Record Robot State with Rerun {#rerun}

[![rerun][c~rerun~docs~badge]][c~rerun~docs]

Rerun records time-indexed robot data and can buffer it in memory. A points log can show chassis vertices, sensor locations, or path samples:

```rust,ignore
fn log_robot_pose() -> Result<(), Box<dyn std::error::Error>> {
    let (recording, storage) = rerun::RecordingStreamBuilder::new("robotics").memory()?;
    recording.set_time_sequence("frame", 42);
    recording.log(
        "robot/chassis",
        &rerun::Points3D::new([(0.0, 0.0, 0.0), (1.0, 0.0, 0.0)]),
    )?;
    let _logged_chunks = storage.take();
    Ok(())
}
```

## Simulate Robot Physics with Rapier {#physics-simulation}

[![rapier2d][c~rapier2d~docs~badge]][c~rapier2d~docs]

Rapier provides 2D and 3D rigid-body physics. This headless example drops a simple robot chassis onto a floor and checks its resting height and speed; see the [physics engines chapter][p~physics-engines] for broader guidance.

```rust,editable
{{#include ../../../crates/cats/science_robotics/examples/simulation_visualization/robot_simulation.rs:example}}
```

## Build an Interactive Environment with Bevy {#game-engine}

[![bevy][c~bevy~docs~badge]][c~bevy~docs]

Bevy provides an ECS-based game engine for interactive robot scenes. The old standalone `bevy_rapier` repository is archived; its integration now lives in the [Rapier repository][bevy_rapier~repo]. Choose a plugin release compatible with the Bevy version in your project.

## Plot Simulation Results {#plot-results}

[![plotters][c~plotters~docs~badge]][c~plotters~docs]

Use Plotters for static plots of numeric traces such as position, speed, or controller error. Its `svg_backend` feature enables portable SVG output; select only the backend features the application needs.

## Other Visualization Tools {#other-tools .skip}

- [`kiss3d`][c~kiss3d~crates.io] provides a lightweight 3D viewer.
- [`urdf-viz`][c~urdf-viz~crates.io] visualizes robot descriptions in URDF format.
- [`zmqRemoteApi`][c~zmqRemoteApi~docs] connects Rust clients to the CoppeliaSim robotics simulator.

## Related Topics {#related-topics .skip}

- [[robotics_frameworks | Robotics Frameworks]].
- [[robot_operating_systems | Robot Operating Systems]].
- [[simulation | Simulation]].
- [[visualization | Visualization]].
- [[aerospace_simulation | Aerospace Simulation]].

For ROS 2 data visualization outside Rust applications, [Foxglove][foxglove~ros2] supports live connections and recorded MCAP files.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
