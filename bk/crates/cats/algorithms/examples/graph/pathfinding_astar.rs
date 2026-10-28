#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Implicit grid pathfinding using the A* algorithm and the `pathfinding`
//! crate.
//!
//! ### What is Implicit Graph Pathfinding?
//! Unlike explicit graph structures that construct all nodes and edges in
//! memory upfront, implicit graph pathfinding generates neighboring states on
//! the fly using a `successors` closure. This is ideal for grid-based maps,
//! mazes, games, and large search spaces where constructing the entire graph
//! in advance is impractical.
//!
//! ### Algorithm Overview & Heuristic Selection
//! - **A* Evaluation**: Uses $f(n) = g(n) + h(n)$ to prioritize search paths.
//! - **Manhattan Distance**: On a 4-connected grid (moving up, down, left,
//!   right), the Manhattan distance $|x_1 - x_2| + |y_1 - y_2|$ represents the
//!   exact minimum distance without obstacles, making it an **admissible**
//!   heuristic.
//! - **Obstacle Avoidance**: Impassable grid cells (walls) are excluded
//!   directly within the `successors` transition function.

use pathfinding::prelude::astar;

/// Demonstrates grid-based A* pathfinding using implicit graph transitions.
///
/// Finds the shortest path on a 5x5 grid from (0, 0) to (4, 4) while navigating
/// around static obstacles.
fn main() {
    /// Represents a discrete 2D grid coordinate `(x, y)` in the search space.
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct Pos(i32, i32);

    impl Pos {
        /// Computes the Manhattan distance heuristic between two grid points.
        ///
        /// Calculates $|x_1 - x_2| + |y_1 - y_2|$, which serves as an
        /// admissible heuristic for 4-connected orthogonal movements.
        fn distance(&self, other: &Self) -> u32 {
            self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
        }

        /// Generates valid adjacent grid positions reachable in a single step.
        ///
        /// Generates orthogonal moves (North, South, East, West), enforces grid
        /// boundary limits [0..4], filters out blocked obstacle cells, and
        /// assigns step cost 1.
        fn successors(&self) -> Vec<(Self, u32)> {
            let &Self(x, y) = self;
            vec![
                Self(x + 1, y),
                Self(x - 1, y),
                Self(x, y + 1),
                Self(x, y - 1),
            ]
            .into_iter()
            // Keep moves strictly within the 5x5 grid boundary.
            .filter(|p| p.0 >= 0 && p.0 <= 4 && p.1 >= 0 && p.1 <= 4)
            // Filter out blocked obstacle coordinates (walls).
            .filter(|p| ![(1, 1), (1, 2), (2, 1)].contains(&(p.0, p.1)))
            // Return valid neighbor state paired with uniform step cost 1.
            .map(|p| (p, 1))
            .collect()
        }
    }

    let start = Pos(0, 0);
    let goal = Pos(4, 4);

    // Run A* pathfinding using the implicit `successors` method and Manhattan
    // distance heuristic.
    let result = astar(
        &start,
        Pos::successors,
        |p| p.distance(&goal),
        |p| p == &goal,
    );

    if let Some((path, cost)) = result {
        println!("Found path with cost {cost}: {path:?}");
        assert_eq!(cost, 8);
        assert_eq!(path.last(), Some(&goal));
    } else {
        panic!("No path found!");
    }
}

// ANCHOR_END: example

/// Runs the `pathfinding` crate A* example by calling `main()`.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_astar_pathfinding_example() {
        main();
    }
}
