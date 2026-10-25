<!-- TODO Reviewed ??-??-2026 -->

# Control Systems

A control system measures a quantity (speed, position, temperature), compares it with a setpoint, and adjusts an actuator to reduce the error. This page covers feedback control with PID, the linear algebra used for state-space models and estimators, and motion planning. Hardware access is covered in [Hardware Integration][p~hardware-integration].

{{#include control_systems.incl.md}}

- Motor Control: Often uses [`embedded-hal`][c~embedded-hal~docs]↗{{hi:embedded-hal}} and specific hardware crates. Motor control often involves low-level hardware interaction using crates based on [`embedded-hal`][c~embedded-hal~docs]↗{{hi:embedded-hal}}.
- Actuator Control: Often uses [`embedded-hal`][c~embedded-hal~docs]↗{{hi:embedded-hal}} and specific hardware crates.
- Motion Planning: [Linear algebra][p~linear-algebra] crates like [`nalgebra`][c~nalgebra~docs]↗{{hi:nalgebra}} and [`alga`][c~alga~docs]↗{{hi:alga}} are foundational.
- [[path_planning | Path Planning]]: Path planning algorithms are often implemented using graph algorithms and search techniques.

## Feedback Control with `pid` {#pid}

[![pid][c~pid~docs~badge]][c~pid~docs]{{hi:pid}}
[![pid~crates.io][c~pid~crates.io~badge]][c~pid~crates.io]
[![pid~repo][c~pid~repo~badge]][c~pid~repo]
[![pid~lib.rs][c~pid~lib.rs~badge]][c~pid~lib.rs]
[![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

[`pid`][c~pid~docs]↗{{hi:pid}} is a small, generic, `no_std` PID (proportional, integral, derivative) controller. Set a target with `Pid::new(setpoint, output_limit)`, configure each term with `p`, `i` and `d` (a gain and a limit), then call `next_control_output(measurement)` once per control period. The limits keep the output, and the integral term, bounded (anti-windup).

The example drives a simulated first-order plant (a motor speed or a heater) toward a setpoint. Because the plant is simulated, the test needs no hardware.

```rust,editable
{{#include ../../../crates/cats/science_robotics/examples/control_systems/pid.rs:example}}
```

For more demanding designs, estimate the hidden state of the system with a Kalman filter (see the filters in [`nalgebra`][c~nalgebra~docs]↗{{hi:nalgebra}}-based crates such as [`adskalman`][adskalman~repo]↗) before feeding it to the controller. Run the loop at a fixed period, on a real-time thread or an [embedded][p~embedded] timer interrupt.

## `nalgebra`: Linear Algebra Library for Control Algorithms {#nalgebra}

[![nalgebra~website][c~nalgebra~website~badge]][c~nalgebra~website] [![nalgebra][c~nalgebra~docs~badge]][c~nalgebra~docs] [![nalgebra~crates.io][c~nalgebra~crates.io~badge]][c~nalgebra~crates.io] [![nalgebra~repo][c~nalgebra~repo~badge]][c~nalgebra~repo] [![nalgebra~lib.rs][c~nalgebra~lib.rs~badge]][c~nalgebra~lib.rs]{{hi:nalgebra}}{{hi:Math}}{{hi:Algebra}}{{hi:Linear}}{{hi:Matrix}}{{hi:Vector}} [![cat~mathematics][cat~mathematics~badge]][cat~mathematics]{{hi:Mathematics}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}} [![cat~science][cat~science~badge]][cat~science]{{hi:Science}} [![cat~wasm][cat~wasm~badge]][cat~wasm]{{hi:WebAssembly}}

General-purpose [linear algebra][p~linear-algebra] library with transformations and statically-sized or dynamically-sized matrices. Use it for state-space models (`x' = Ax + Bu`), coordinate transforms and the matrices of estimators.

## `k`: Kinematics Library for Robotics {#k}

[![k][c~k~docs~badge]][c~k~docs] [![k~crates.io][c~k~crates.io~badge]][c~k~crates.io] [![k~repo][c~k~repo~badge]][c~k~repo] [![k~lib.rs][c~k~lib.rs~badge]][c~k~lib.rs]{{hi:k}}{{hi:Robotics}}{{hi:Kinematics}}{{hi:Ik}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}

`k` is for kinematics.

## Path Planning {#path-planning .skip}

Collision avoidance, trajectory optimization.

## Pathfinding Algorithms for Navigation with `pathfinding` {#pathfinding}

[![pathfinding~website][c~pathfinding~website~badge]][c~pathfinding~website] [![pathfinding][c~pathfinding~docs~badge]][c~pathfinding~docs] [![pathfinding~crates.io][c~pathfinding~crates.io~badge]][c~pathfinding~crates.io] [![pathfinding~repo][c~pathfinding~repo~badge]][c~pathfinding~repo] [![pathfinding~lib.rs][c~pathfinding~lib.rs~badge]][c~pathfinding~lib.rs]{{hi:pathfinding}}{{hi:Astar}}{{hi:Graph}}{{hi:Flow}}{{hi:Dijkstra}}{{hi:Shortest-path}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}

[`pathfinding`][c~pathfinding~docs]↗{{hi:pathfinding}} offers pathfinding, flow, and graph algorithms.

{{#example pathfinding}}

## `rrt` {#rrt}

[![rrt][c~rrt~docs~badge]][c~rrt~docs] [![rrt~crates.io][c~rrt~crates.io~badge]][c~rrt~crates.io] [![rrt~repo][c~rrt~repo~badge]][c~rrt~repo] [![rrt~lib.rs][c~rrt~lib.rs~badge]][c~rrt~lib.rs]{{hi:rrt}}{{hi:Search}}{{hi:Path-finding}}{{hi:Robotics}}{{hi:rrt}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}

[`rrt`][c~rrt~docs]↗{{hi:rrt}} stands for Rapidly-exploring Random Tree library. Path finding using dual-RRT connect.

## `openrr-planner` {#openrr-planner}

[![openrr-planner][c~openrr-planner~docs~badge]][c~openrr-planner~docs] [![openrr-planner~crates.io][c~openrr-planner~crates.io~badge]][c~openrr-planner~crates.io] [![openrr-planner~repo][c~openrr-planner~repo~badge]][c~openrr-planner~repo] [![openrr-planner~lib.rs][c~openrr-planner~lib.rs~badge]][c~openrr-planner~lib.rs]{{hi:openrr-planner}}{{hi:Robot}}{{hi:Robotics}}{{hi:Pathplanning}} [![cat~science::robotics][cat~science::robotics~badge]][cat~science::robotics]{{hi:Robotics}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}

Collision avoidance path planning for robotics.

## `rs-opw-kinematics` {#rs-opw-kinematics}

[![rs-opw-kinematics][c~rs-opw-kinematics~docs~badge]][c~rs-opw-kinematics~docs] [![rs-opw-kinematics~crates.io][c~rs-opw-kinematics~crates.io~badge]][c~rs-opw-kinematics~crates.io] [![rs-opw-kinematics~repo][c~rs-opw-kinematics~repo~badge]][c~rs-opw-kinematics~repo] [![rs-opw-kinematics~lib.rs][c~rs-opw-kinematics~lib.rs~badge]][c~rs-opw-kinematics~lib.rs]{{hi:rs-opw-kinematics}}{{hi:Collisions}}{{hi:Robotics}}{{hi:Kinematics}}{{hi:Path-planning}}{{hi:Opw}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}} [![cat~science::robotics][cat~science::robotics~badge]][cat~science::robotics]{{hi:Robotics}}

Inverse and forward kinematics for 6-axis robots with a parallel base and spherical wrist.

## Related Topics {#related-topics .skip}

- [Hardware Integration][p~hardware-integration].
- [[embedded | Embedded Systems]].
- Path finding: see [Artificial Intelligence for Robotics][p~robotics-ai].
- [Robot frameworks](./robotics_frameworks.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
