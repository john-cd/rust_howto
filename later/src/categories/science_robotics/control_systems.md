<!-- TODO Reviewed ??-??-2026 -->

# Control Systems

A control system measures a quantity (speed, position, temperature), compares it with a setpoint, and adjusts an actuator to reduce the error. This page covers feedback control with PID, the linear algebra used for state-space models and estimators, and motion planning. Hardware access is covered in [Hardware Integration][p~hardware-integration].

{{#include control_systems.incl.md}}

- Motor Control: Often uses [`embedded-hal`][c~embedded-hal~docs]↗{{hi:embedded-hal}} and specific hardware crates. Motor control often involves low-level hardware interaction using crates based on [`embedded-hal`][c~embedded-hal~docs]↗{{hi:embedded-hal}}.
- Actuator Control: Often uses [`embedded-hal`][c~embedded-hal~docs]↗{{hi:embedded-hal}} and specific hardware crates.
- Motion Planning: [Linear algebra][p~linear-algebra] crates like [`nalgebra`][c~nalgebra~docs]↗{{hi:nalgebra}} and [`alga`][c~alga~docs]↗{{hi:alga}} are foundational.
- [[path_planning | Path Planning]]: Path planning algorithms are often implemented using graph algorithms and search techniques.

## Feedback Control with `pid` {#pid}

{{#badges pid no-std="No standard library"}}

[`pid`][c~pid~docs]↗{{hi:pid}} is a small, generic, `no_std` PID (proportional, integral, derivative) controller. Set a target with `Pid::new(setpoint, output_limit)`, configure each term with `p`, `i` and `d` (a gain and a limit), then call `next_control_output(measurement)` once per control period. The limits keep the output, and the integral term, bounded (anti-windup).

The example drives a simulated first-order plant (a motor speed or a heater) toward a setpoint. Because the plant is simulated, the test needs no hardware.

`next_control_output` takes a measurement, not an elapsed time: the integral term accumulates error once per call, and the derivative term uses the change in measurement between calls. Tune the gains for the chosen fixed sampling period and retune or rescale them if that period changes. Real controllers must also account for timing jitter, actuator limits, and the behavior of the actual plant.

```rust,editable
{{#include ../../../crates/cats/science_robotics/examples/control_systems/pid.rs:example}}
```

For more demanding designs, estimate the hidden state of the system with a Kalman filter (see the filters in [`nalgebra`][c~nalgebra~docs]↗{{hi:nalgebra}}-based crates such as [`adskalman`][adskalman~repo]↗) before feeding it to the controller. Run the loop at a fixed period, on a real-time thread or an [embedded][p~embedded] timer interrupt.

## `nalgebra`: Linear Algebra Library for Control Algorithms {#nalgebra}

[![nalgebra~website][c~nalgebra~website~badge]][c~nalgebra~website] [![nalgebra][c~nalgebra~docs~badge]][c~nalgebra~docs] [![nalgebra~crates.io][c~nalgebra~crates.io~badge]][c~nalgebra~crates.io] [![nalgebra~repo][c~nalgebra~repo~badge]][c~nalgebra~repo] [![nalgebra~lib.rs][c~nalgebra~lib.rs~badge]][c~nalgebra~lib.rs]{{hi:nalgebra}}{{hi:Math}}{{hi:Algebra}}{{hi:Linear}}{{hi:Matrix}}{{hi:Vector}} [![cat~mathematics][cat~mathematics~badge]][cat~mathematics]{{hi:Mathematics}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}} [![cat~science][cat~science~badge]][cat~science]{{hi:Science}} [![cat~wasm][cat~wasm~badge]][cat~wasm]{{hi:WebAssembly}}

General-purpose [linear algebra][p~linear-algebra] library with transformations and statically-sized or dynamically-sized matrices. Use it for state-space models (`x' = Ax + Bu`), coordinate transforms and the matrices of estimators.

## `k`: Kinematics Library for Robotics {#k}

{{#badges k algorithms=Algorithms}}{{hi:Robotics}}{{hi:Kinematics}}{{hi:Ik}}

`k` is for kinematics.

## Geometry and Numerical Libraries {#geometry-and-numerical-libraries}

- `ncollide2d` and `ncollide3d` provide two- and three-dimensional collision detection.
- [`kdtree`][c~kdtree~crates.io] provides k-dimensional trees for nearest-neighbor queries.
- [`static-math`][c~static-math~docs] provides mathematical operations over fixed-size arrays.
- [`ndarray`][c~ndarray~docs] provides n-dimensional tensor arithmetic.

## Linear Algebra with `faer` {#faer}

[`faer`][faer~docs] is a general-purpose Rust linear algebra library focused on medium and large matrices and matrix decompositions. See also [[additional_numeric_types | Additional Numeric Types]] and [[linear_algebra | Linear Algebra]].

## Path Planning {#path-planning .skip}

Collision avoidance, trajectory optimization.

## Pathfinding Algorithms for Navigation with `pathfinding` {#pathfinding}

[![pathfinding~website][c~pathfinding~website~badge]][c~pathfinding~website] [![pathfinding][c~pathfinding~docs~badge]][c~pathfinding~docs] [![pathfinding~crates.io][c~pathfinding~crates.io~badge]][c~pathfinding~crates.io] [![pathfinding~repo][c~pathfinding~repo~badge]][c~pathfinding~repo] [![pathfinding~lib.rs][c~pathfinding~lib.rs~badge]][c~pathfinding~lib.rs]{{hi:pathfinding}}{{hi:Astar}}{{hi:Graph}}{{hi:Flow}}{{hi:Dijkstra}}{{hi:Shortest-path}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}

[`pathfinding`][c~pathfinding~docs]↗{{hi:pathfinding}} offers pathfinding, flow, and graph algorithms.

```rust,editable
{{#include ../../../../bk/crates/cats/algorithms/examples/graph/pathfinding_astar.rs:example}}
```

## `rrt` {#rrt}

{{#badges rrt algorithms=Algorithms}}{{hi:Search}}{{hi:Path-finding}}{{hi:Robotics}}{{hi:rrt}}

[`rrt`][c~rrt~docs]↗{{hi:rrt}} stands for Rapidly-exploring Random Tree library. Path finding using dual-RRT connect.

## `openrr-planner` {#openrr-planner}

{{#badges openrr-planner science::robotics=Robotics algorithms=Algorithms}}{{hi:Robot}}{{hi:Robotics}}{{hi:Pathplanning}}

Collision avoidance path planning for robotics.

## `rs-opw-kinematics` {#rs-opw-kinematics}

{{#badges rs-opw-kinematics algorithms=Algorithms science::robotics=Robotics}}{{hi:Collisions}}{{hi:Robotics}}{{hi:Kinematics}}{{hi:Path-planning}}{{hi:Opw}}

Inverse and forward kinematics for 6-axis robots with a parallel base and spherical wrist.

## Related Topics {#related-topics .skip}

- [Hardware Integration][p~hardware-integration].
- [[embedded | Embedded Systems]].
- Path finding: see [Artificial Intelligence for Robotics][p~robotics-ai].
- [Robot frameworks](./robotics_frameworks.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
