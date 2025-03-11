# Control Systems

Motion control, feedback loops, PID controllers.

{{#include control_systems.incl.md}}

- Motor Control: Often uses [`embedded-hal`][c-embedded_hal]⮳{{hi:embedded-hal}} and specific hardware crates. Motor control often involves low-level hardware interaction using crates based on [`embedded-hal`][c-embedded_hal]⮳{{hi:embedded-hal}}.
- Actuator Control: Often uses [`embedded-hal`][c-embedded_hal]⮳{{hi:embedded-hal}} and specific hardware crates.
- Motion Planning: [Linear algebra][p-linear-algebra] crates like [`nalgebra`][c-nalgebra]⮳{{hi:nalgebra}} and [`alga`][c-alga]⮳{{hi:alga}} are foundational.
- [[path_planning | Path Planning]]: Path planning algorithms are often implemented using graph algorithms and search techniques.

## `nalgebra`: Linear algebra library for control algorithms {#nalgebra}

## `k`: Kinematics library for robotics {#k}

## Path Planning {#skip}

Collision avoidance, trajectory optimization.

## `pathfinding`: Pathfinding algorithms for navigation {#pathfinding}

Pathfinding library for rust

## `rrt` {#rrt}

Rapidly-exploring Random Tree library.

## `openrr-planner` {#openrr-planner}

Collision avoidance path planning.

## `rs-opw-kinematics` {#rs-opw-kinematics}

Analytical inverse and forward kinematics for the 6DOF robots with spherical wrist.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO write
Review `pid`
</div>
