<!-- TODO Reviewed ??-??-2026 -->

# Robot Operating Systems (ROS)

{{#include robot_operating_systems.incl.md}}

ROS connects robot nodes through topics, services, and actions. Choose a ROS 2 client for new systems; [ROS 1 Noetic reached end of life on May 31, 2025][ros1~eol]. Rust clients differ in their dependence on the native ROS libraries and in the message types and middleware features they support.

## Pick a ROS Client

The release snapshot below was checked on October 29, 2026. A recent release shows activity, but does not guarantee compatibility with every ROS distribution or DDS implementation.

| Client                            | ROS version and approach                                                        | Latest stable release checked | Integration and status                                                                                                  |
| --------------------------------- | ------------------------------------------------------------------------------- | ----------------------------- | ----------------------------------------------------------------------------------------------------------------------- |
| [`r2r`][r2r~setup]                | ROS 2; bindings to installed `rcl` libraries and generated message support      | 0.9.7, September 13, 2026     | Used by the recipe below; source a compatible ROS installation and install libclang.                                    |
| [`rclrs`][ros2-rust~repo]         | ROS 2; Rust client built on `rcl` in the ros2-rust project                      | 0.8.0, September 23, 2026     | Integrates with ROS workspaces, `colcon`, and `cargo-ament-build`; its API and distribution support are still evolving. |
| [`ros2-client`][ros2-client~repo] | ROS 2; native Rust client with RustDDS by default and an optional Zenoh backend | 0.12.1, October 26, 2026      | Avoids the native `rcl` client stack; check message generation, QoS, security, and ROS interoperability support.        |
| [`rosrust`][c~rosrust~repo]       | ROS 1; native Rust client                                                       | 0.9.12, February 3, 2025      | For existing ROS 1 deployments; the ROS 1 ecosystem is end of life.                                                     |
| [`roslibrust`][roslibrust~repo]   | Async Rust abstraction with ROS 1, ROS 2, rosbridge, and Zenoh backends         | 0.26.0, September 25, 2026    | Select the backend and its prerequisites explicitly; rosbridge needs a bridge server and adds a protocol hop.           |

ROS applications are composed of nodes that exchange messages on topics, make request-response calls through services, or run long-lived actions. Select a client based on whether the project already uses a ROS installation and `rcl`, needs a pure-Rust middleware stack, or communicates through a bridge. The table's release dates are a snapshot; verify current package and distribution support before choosing.

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

## Call Services and Integrate Builds

A topic broadcasts a stream; a service pairs one request with one response. Use the selected client's generated service types and keep its executor or spin loop running while waiting for the response. Use bounded waits so an absent server cannot hang application shutdown. For long-running, cancelable work, evaluate ROS actions and the client's action support rather than holding a service request open.

For `rclrs`, follow the [ros2-rust workspace guide][ros2-rust~repo]: source the ROS environment, install the documented `colcon` extensions including `cargo-ament-build`, and build the workspace with `colcon build`. Source the resulting `install` setup before running nodes. A standalone Cargo command does not replace ROS message generation and environment setup. The `r2r` example instead uses its own Cargo build-time generation from the installed message packages.

[`safe_drive`][c~safe_drive~docs] offers another ROS 2 binding approach. Check its supported distribution and message-generation workflow before mixing it with a different client's generated types.

## Other Crates {#other-crates .skip}

- [`ros2_rust`][ros2_rust~repo]↗{{hi:ros2_rust}} - Rust bindings for ROS2.
- [`roslibrust`][c~roslibrust~crates.io]↗{{hi:roslibrust}} - Multiple async ROS backends, including rosbridge.
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
