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

ROS 1 was the original system, widely used in academia and industry. However, it had some limitations with real-time performance.ROS 2 is the newer version designed to address these limitations. It emphasizes performance, security, and support for multi-robot systems.

- Rust in ROS:

While ROS traditionally favored [Python][p~python] and C++, Rust is gradually gaining traction due to its strong memory safety, [concurrency][p~concurrency] features, and [performance][p~performance] benefits. Libraries like [`rosrust`][c~rosrust~docs]↗{{hi:rosrust}} and `r2r` bring ROS functionality to Rust, enabling developers to create safe and efficient robotics applications.

[`rosrust`][c~rosrust~docs]↗{{hi:rosrust}} provides a client library for interacting with ROS. `ros_control_rs` aims to provide Rust bindings for ROS Control.

## `rosrust`: Pure Rust Implementation of a ROS Client Library {#rosrust}

[![rosrust][c~rosrust~docs~badge]][c~rosrust~docs] [![rosrust~crates.io][c~rosrust~crates.io~badge]][c~rosrust~crates.io] [![rosrust~repo][c~rosrust~repo~badge]][c~rosrust~repo] [![rosrust~lib.rs][c~rosrust~lib.rs~badge]][c~rosrust~lib.rs]{{hi:rosrust}}

[`rosrust`][c~rosrust~docs]↗{{hi:rosrust}} is a pure Rust implementation of a ROS client library.

## `r2r`: Minimal ROS2 Rust Bindings {#r2r}

[![r2r][c~r2r~docs~badge]][c~r2r~docs] [![r2r~crates.io][c~r2r~crates.io~badge]][c~r2r~crates.io] [![r2r~repo][c~r2r~repo~badge]][c~r2r~repo] [![r2r~lib.rs][c~r2r~lib.rs~badge]][c~r2r~lib.rs]{{hi:r2r}}

[`r2r`][c~r2r~docs]↗{{hi:r2r}} offers minimal ROS2 Rust bindings. Easy to use, runtime-agnostic, async rust bindings for ROS2.

```rust,editable
{{#include ../../../crates/cats/science_robotics/examples/robot_operating_systems/robotics.rs:example}}
```

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

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[robotics: organize](https://github.com/john-cd/rust_howto/issues/477)
review in depth.
review ROS related libraries.
</div>
