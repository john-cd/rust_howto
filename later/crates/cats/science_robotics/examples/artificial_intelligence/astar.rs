#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Plan a robot's path on an occupancy grid with A*.

use pathfinding::prelude::astar;

/// Grid size (cells are addressed as `(x, y)`).
const WIDTH: i32 = 5;
const HEIGHT: i32 = 5;

/// Cells that the robot cannot enter.
const WALLS: [(i32, i32); 4] = [(1, 0), (1, 1), (1, 2), (3, 3)];

/// Returns the free neighbors of a cell; moving costs 1.
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

/// Finds the shortest path from the top-left to the bottom-right corner.
fn main() {
    let start = (0, 0);
    let goal = (4, 4);
    // The Manhattan distance never overestimates the remaining cost.
    let heuristic =
        |&(x, y): &(i32, i32)| ((goal.0 - x).abs() + (goal.1 - y).abs()) as u32;

    let (path, cost) = astar(
        &start,
        |&cell| successors(cell),
        heuristic,
        |&cell| cell == goal,
    )
    .expect("the goal is reachable");

    println!("{path:?}");
    // The wall at x = 1 forces a detour through (1, 3).
    assert_eq!(cost, 8);
    assert_eq!(path.len(), 9);
}
// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the astar example through its entry point.
    #[test]
    fn test_astar_example_entry_point() {
        main();
    }
}
