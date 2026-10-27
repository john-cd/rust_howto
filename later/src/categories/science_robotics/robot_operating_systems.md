<!-- TODO Reviewed ??-??-2026 -->

# Robot Operating Systems (ROS)

{{#include robot_operating_systems.incl.md}}

Topics: Middleware, communication protocols, distributed systems.

A Robot Operating System (ROS) is a middleware framework designed to help build and manage robot software systems. Its primary purpose is to simplify the complexities of robotic development by providing tools and libraries that support tasks like hardware abstraction, communication between different parts of the robot (such as sensors and actuators), and software reuse.

Here are some key aspects of Robot Operating Systems:

- Core Functionality:

Message Passing: ROS facilitates communication between various components (nodes) of a robot system using a publish-subscribe messaging paradigm. For example, a sensor might publish data that a motion planner subscribes to.

Hardware Abstraction: It enables developers to write code that works across different hardware platforms, like [sensors][p~sensors], cameras, and motors.

- Modular Design:

ROS systems are composed of small, independent units called nodes, each performing a specific task. For instance, one node could be responsible for navigation, another for path planning, and another for sensor integration. This modularity promotes flexibility and scalability.

- Development Tools:

Tools like [`rviz`][c~rviz~crates.io]↗{{hi:rviz}} (for [visualization][p~visualization]) and `Gazebo`{{hi:Gazebo}} (for simulation) help developers test and debug their robotic systems without needing the actual hardware.

- Supported Platforms:

ROS supports various programming languages, such as Python, C++, and now Rust. Developers can choose the language that best fits their project.

- Versioning:

ROS 1 was the original system, widely used in academia and industry. ROS 2 is the newer version designed to address its limitations. It emphasizes performance, security, and support for multi-robot systems. New examples in this chapter target ROS 2.

- Rust in ROS:

While ROS traditionally favored [Python][p~python] and C++, Rust offers strong memory safety, [concurrency][p~concurrency] features, and [performance][p~performance] benefits. [`r2r`][c~r2r~docs] provides bindings to the ROS 2 client libraries.

## `rosrust`: Pure Rust Implementation of a ROS Client Library {#rosrust}

[![rosrust][c~rosrust~docs~badge]][c~rosrust~docs] [![rosrust~crates.io][c~rosrust~crates.io~badge]][c~rosrust~crates.io] [![rosrust~repo][c~rosrust~repo~badge]][c~rosrust~repo] [![rosrust~lib.rs][c~rosrust~lib.rs~badge]][c~rosrust~lib.rs]{{hi:rosrust}}

[`rosrust`][c~rosrust~docs]↗{{hi:rosrust}} targets ROS 1. The former ROS 1 example and dependencies have been replaced with the ROS 2 recipe below; `rosrust` is not enabled by this workspace.

## `r2r`: Minimal ROS2 Rust Bindings {#r2r}

[![r2r][c~r2r~docs~badge]][c~r2r~docs] [![r2r~crates.io][c~r2r~crates.io~badge]][c~r2r~crates.io] [![r2r~repo][c~r2r~repo~badge]][c~r2r~repo] [![r2r~lib.rs][c~r2r~lib.rs~badge]][c~r2r~lib.rs]{{hi:r2r}}

[`r2r`][c~r2r~docs]↗{{hi:r2r}} offers runtime-agnostic ROS 2 bindings and typed message streams. This recipe uses `r2r` rather than a pure Rust DDS client so it can generate `std_msgs` bindings from the sourced ROS 2 installation and use its installed middleware. See the [upstream setup instructions][r2r~setup] for supported ROS distributions and prerequisites.

The upstream project documents Linux, macOS, and Windows support. There is no Windows exclusion in this example: each platform needs a compatible ROS 2 installation, its development environment loaded in the current shell, and libclang available for binding generation. Platform and ROS distribution combinations must follow the upstream instructions; enabling the feature alone does not install ROS.

The `science_robotics` crate's `ros2` feature is off by default, and the example has `required-features = ["ros2"]`. Default builds skip the ROS dependencies and example. Builds with `--all-features` require the ROS environment too. The dependency versions are declared in `later/crates/Cargo.toml` and inherited by the owning crate.

This node publishes five `std_msgs/String` greetings on `/rust_howto/hello` and receives them through a local subscriber. DDS discovery can take time, so each greeting is retried until received, with a ten-second timeout. `spin_once` transfers ROS events into the subscription stream, which is then polled without blocking on an async executor. Duplicate or unrelated greetings are ignored.

```rust,ignore
{{#include ../../../crates/cats/science_robotics/examples/robot_operating_systems/ros2_pubsub.rs:example}}
```

For example, in a Linux shell with ROS 2 Jazzy and libclang installed, run from `later/crates`:

```bash
source /opt/ros/jazzy/setup.bash
cargo run -p science_robotics --example robot_operating_systems --features ros2
```

On macOS or Windows, load the installed ROS 2 distribution's environment using its platform-specific setup instructions, then use the same Cargo command. No ROS 1 master (`roscore`) is involved. An external ROS 2 observer in the same ROS domain can also receive the greetings:

```bash
ros2 topic echo /rust_howto/hello std_msgs/msg/String
```

The existing example test is ignored because it needs ROS 2 and working DDS discovery. To run it explicitly in a configured environment:

```bash
cargo test -p science_robotics --example robot_operating_systems --features ros2 -- --ignored
```

If the example times out, check the selected ROS domain, middleware configuration, and local discovery/firewall settings.

## `safe_drive` {#safe_drive}

Formally specified Rust bindings for ROS2.

## Other Crates {#other-crates .skip}

- [`ros2_rust`][ros2_rust~repo]↗{{hi:ros2_rust}} - Rust bindings for ROS2.
- [`roslibrust`][c~roslibrust~crates.io]↗{{hi:roslibrust}} - Pure Rust implementation of a rosbridge client.
- [`rclrust`][c~rclrust~crates.io]↗{{hi:rclrust}} - Yet another ROS2 Rust client.
- [`RustDDS`][c~rustdds~crates.io]↗{{hi:RustDDS}} - Rust implementation of Data Distribution Service.
- [`rosbag`][c~rosbag~crates.io]↗{{hi:rosbag}} - Reading rosbag files in pure Rust.
- [`rustros_tf`][c~rustros_tf~crates.io]↗{{hi:rustros_tf}} - A rust implementation of the Tf library.
- [`ros_pointcloud2`][c~ros_pointcloud2~docs]{{hi:ros_pointcloud2}} - The safe way of using PointCloud2 messages in ROS1 and ROS2.
- [`optimization-engine`][c~optimization-engine~docs]{{hi:optimization-engine}} - Fast & Accurate Embedded Optimization for next-generation Robotics and Autonomous Systems.
- [`transforms`][c~transforms~crates.io]↗{{hi:transforms}} - A minimal and stand-alone crate inspired by the ROS2 tf library, but not dependent on ROS or middleware.

## Related Topics {#related-topics .skip}

- Distributed middleware.
- Robotics communication protocols.
- Simulation and systems integration.
- [Robot frameworks](./robotics_frameworks.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

[r2r~setup]: https://github.com/sequenceplanner/r2r#how-to-use

<div class="hidden">
  [robotics: organize](https://github.com/john-cd/rust_howto/issues/477)
  review in depth.
  review ROS related libraries.
</div>
