# Control Systems

Motion control, feedback loops, PID controllers.

{{#include control_systems.incl.md}}

- Motor Control: Often uses [`embedded-hal`][c~embedded-hal~docs]↗{{hi:embedded-hal}} and specific hardware crates. Motor control often involves low-level hardware interaction using crates based on [`embedded-hal`][c~embedded-hal~docs]↗{{hi:embedded-hal}}.
- Actuator Control: Often uses [`embedded-hal`][c~embedded-hal~docs]↗{{hi:embedded-hal}} and specific hardware crates.
- Motion Planning: [Linear algebra][p~linear-algebra] crates like [`nalgebra`][c~nalgebra~docs]↗{{hi:nalgebra}} and [`alga`][c~alga~docs]↗{{hi:alga}} are foundational.
- [[path_planning | Path Planning]]: Path planning algorithms are often implemented using graph algorithms and search techniques.

## `nalgebra`: Linear Algebra Library for Control Algorithms {#nalgebra}

[![nalgebra~website][c~nalgebra~website~badge]][c~nalgebra~website] [![nalgebra][c~nalgebra~docs~badge]][c~nalgebra~docs] [![nalgebra~crates.io][c~nalgebra~crates.io~badge]][c~nalgebra~crates.io] [![nalgebra~github][c~nalgebra~github~badge]][c~nalgebra~github] [![nalgebra~lib.rs][c~nalgebra~lib.rs~badge]][c~nalgebra~lib.rs]{{hi:nalgebra}}{{hi:Math}}{{hi:Algebra}}{{hi:Linear}}{{hi:Matrix}}{{hi:Vector}} [![cat~mathematics][cat~mathematics~badge]][cat~mathematics]{{hi:Mathematics}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}} [![cat~science][cat~science~badge]][cat~science]{{hi:Science}} [![cat~wasm][cat~wasm~badge]][cat~wasm]{{hi:WebAssembly}}

General-purpose [linear algebra][p~linear-algebra] library with transformations and statically-sized or dynamically-sized matrices.

## `k`: Kinematics Library for Robotics {#k}

[![k][c~k~docs~badge]][c~k~docs] [![k~crates.io][c~k~crates.io~badge]][c~k~crates.io] [![k~github][c~k~github~badge]][c~k~github] [![k~lib.rs][c~k~lib.rs~badge]][c~k~lib.rs]{{hi:k}}{{hi:Robotics}}{{hi:Kinematics}}{{hi:Ik}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}

`k` is for kinematics.

## Path Planning {#skip}

Collision avoidance, trajectory optimization.

## Pathfinding Algorithms for Navigation with `pathfinding` {#pathfinding}

[![pathfinding~website][c~pathfinding~website~badge]][c~pathfinding~website] [![pathfinding][c~pathfinding~docs~badge]][c~pathfinding~docs] [![pathfinding~crates.io][c~pathfinding~crates.io~badge]][c~pathfinding~crates.io] [![pathfinding~github][c~pathfinding~github~badge]][c~pathfinding~github] [![pathfinding~lib.rs][c~pathfinding~lib.rs~badge]][c~pathfinding~lib.rs]{{hi:pathfinding}}{{hi:Astar}}{{hi:Graph}}{{hi:Flow}}{{hi:Dijkstra}}{{hi:Shortest-path}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}

[`pathfinding`][c~pathfinding~docs]↗{{hi:pathfinding}} offers pathfinding, flow, and graph algorithms.

{{#example pathfinding}}

## `rrt` {#rrt}

[![rrt][c~rrt~docs~badge]][c~rrt~docs] [![rrt~crates.io][c~rrt~crates.io~badge]][c~rrt~crates.io] [![rrt~github][c~rrt~github~badge]][c~rrt~github] [![rrt~lib.rs][c~rrt~lib.rs~badge]][c~rrt~lib.rs]{{hi:rrt}}{{hi:Search}}{{hi:Path-finding}}{{hi:Robotics}}{{hi:rrt}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}

[`rrt`][c~rrt~docs]↗{{hi:rrt}} stands for Rapidly-exploring Random Tree library. Path finding using dual-RRT connect.

## `openrr-planner` {#openrr-planner}

[![openrr-planner][c~openrr-planner~docs~badge]][c~openrr-planner~docs] [![openrr-planner~crates.io][c~openrr-planner~crates.io~badge]][c~openrr-planner~crates.io] [![openrr-planner~github][c~openrr-planner~github~badge]][c~openrr-planner~github] [![openrr-planner~lib.rs][c~openrr-planner~lib.rs~badge]][c~openrr-planner~lib.rs]{{hi:openrr-planner}}{{hi:Robot}}{{hi:Robotics}}{{hi:Pathplanning}} [![cat~science::robotics][cat~science::robotics~badge]][cat~science::robotics]{{hi:Robotics}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}

Collision avoidance path planning for robotics.

## `rs-opw-kinematics` {#rs-opw-kinematics}

[![rs-opw-kinematics][c~rs-opw-kinematics~docs~badge]][c~rs-opw-kinematics~docs] [![rs-opw-kinematics~crates.io][c~rs-opw-kinematics~crates.io~badge]][c~rs-opw-kinematics~crates.io] [![rs-opw-kinematics~github][c~rs-opw-kinematics~github~badge]][c~rs-opw-kinematics~github] [![rs-opw-kinematics~lib.rs][c~rs-opw-kinematics~lib.rs~badge]][c~rs-opw-kinematics~lib.rs]{{hi:rs-opw-kinematics}}{{hi:Collisions}}{{hi:Robotics}}{{hi:Kinematics}}{{hi:Path-planning}}{{hi:Opw}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}} [![cat~science::robotics][cat~science::robotics~badge]][cat~science::robotics]{{hi:Robotics}}

Inverse and forward kinematics for 6-axis robots with a parallel base and spherical wrist.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1208)
Review `pid`
</div>
