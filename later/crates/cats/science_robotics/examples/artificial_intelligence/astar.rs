#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Robot path planning on an occupancy grid using the A* algorithm.
//!
//! ### Occupancy Grids in Robotics
//! An occupancy grid discretizes a continuous 2D environment into a matrix of
//! uniform cells, where each cell is either free space or occupied by an
//! obstacle (such as a wall or equipment).
//!
//! ### Path Planning Algorithm & Heuristic
//! - **A* Optimization**: Computes the collision-free path with minimal total
//!   cost from start position `(0, 0)` to target destination `(4, 4)`.
//! - **Admissible Heuristic**: Uses Manhattan distance $|x_{\text{goal}} - x| +
//!   |y_{\text{goal}} - y|$. In a 4-connected grid (cardinal moves: up, down,
//!   left, right), Manhattan distance equals the theoretical minimum number of
//!   steps needed on an unobstructed grid, ensuring the heuristic is strictly
//!   **admissible** and guarantees an optimal path.
//! - **Collision Avoidance**: Impassable wall cells are filtered out during
//!   neighbor expansion (`successors`).

use pathfinding::prelude::astar;

/// Horizontal grid width in cell units along the X-axis.
const WIDTH: i32 = 5;

/// Vertical grid height in cell units along the Y-axis.
const HEIGHT: i32 = 5;

/// Static obstacles representing occupied wall cells that the robot cannot
/// traverse.
const WALLS: [(i32, i32); 4] = [(1, 0), (1, 1), (1, 2), (3, 3)];

/// Computes accessible non-blocked neighbor cells reachable in one step.
///
/// Evaluates 4-connected cardinal movements (dx, dy), checks boundaries
/// against grid limits, rejects cells occupied by walls, and assigns a uniform
/// movement cost of 1 per step.
fn successors((x, y): (i32, i32)) -> Vec<((i32, i32), u32)> {
    [(1, 0), (-1, 0), (0, 1), (0, -1)]
        .into_iter()
        .map(|(dx, dy)| (x + dx, y + dy))
        .filter(|&(nx, ny)| {
            (0..WIDTH).contains(&nx)
                && (0..HEIGHT).contains(&ny)
                && !WALLS.contains(&(nx, ny))
        })
        .map(|cell| (cell, 1))
        .collect()
}

/// Finds the shortest collision-free path from `(0, 0)` to `(4, 4)`.
fn main() {
    let start = (0, 0);
    let goal = (4, 4);

    // The Manhattan distance heuristic never overestimates actual remaining
    // cost on a 4-connected grid.
    let heuristic =
        |&(x, y): &(i32, i32)| ((goal.0 - x).abs() + (goal.1 - y).abs()) as u32;

    // Execute A* path planning using state transitions and heuristic
    // estimation.
    let (path, cost) = astar(
        &start,
        |&cell| successors(cell),
        heuristic,
        |&cell| cell == goal,
    )
    .expect("the goal is reachable");

    println!("Planned trajectory path: {path:?}");

    // The wall obstacle at x = 1 forces a detour through (1, 3).
    assert_eq!(cost, 8);
    assert_eq!(path.len(), 9);
}

// ANCHOR_END: example

/// Runs the robotics pathfinding example by calling `main()`.
pub fn run() -> anyhow::Result<()> {
    main();
    Ok(())
}

#[cfg(test)]
/// Unit tests for the robotics A* pathfinding example.
mod tests {
    use super::*;

    /// Runs the A* robotics pathfinding example and validates path generation.
    #[test]
    fn test_astar_example_entry_point() {
        main();
    }
}
