#![allow(dead_code)]
// ANCHOR: example
use pathfinding::prelude::astar;

/// Demonstrates grid-based A* pathfinding using the `pathfinding` crate.
///
/// This example finds the shortest path on a 2D grid from (0, 0) to (4, 4),
/// avoiding some blocked cells.
fn main() {
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct Pos(i32, i32);

    impl Pos {
        fn distance(&self, other: &Pos) -> u32 {
            (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as u32
        }

        fn successors(&self) -> Vec<(Pos, u32)> {
            let &Pos(x, y) = self;
            vec![Pos(x + 1, y), Pos(x - 1, y), Pos(x, y + 1), Pos(x, y - 1)]
                .into_iter()
                .filter(|p| p.0 >= 0 && p.0 <= 4 && p.1 >= 0 && p.1 <= 4)
                .filter(|p| ![(1, 1), (1, 2), (2, 1)].contains(&(p.0, p.1)))
                .map(|p| (p, 1))
                .collect()
        }
    }

    let start = Pos(0, 0);
    let goal = Pos(4, 4);

    let result = astar(
        &start,
        |p| p.successors(),
        |p| p.distance(&goal),
        |p| p == &goal,
    );

    if let Some((path, cost)) = result {
        println!("Found path with cost {}: {:?}", cost, path);
        assert_eq!(cost, 8);
        assert_eq!(path.last(), Some(&goal));
    } else {
        panic!("No path found!");
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
