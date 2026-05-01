#![allow(dead_code)]
// ANCHOR: example
use std::cmp::Ordering;

// A struct to represent a software version.
// The `Eq` trait has no methods, thus we can derive it automatically.
#[derive(Debug, Eq)]
struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

// Implement `PartialEq` to define when two versions are equal.
impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.major == other.major
            && self.minor == other.minor
            && self.patch == other.patch
    }
}

// Implement `PartialOrd`. Since our ordering is total, `partial_cmp`
// can simply wrap the result of `cmp` in `Some`.
impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Implement `Ord` to define the total ordering of versions.
// It implements a lexicographical comparison.
impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare major versions first.
        match self.major.cmp(&other.major) {
            Ordering::Equal => {
                // If major versions are equal, compare minor versions.
                match self.minor.cmp(&other.minor) {
                    Ordering::Equal => {
                        // If minor versions are also equal, compare patch
                        // versions.
                        self.patch.cmp(&other.patch)
                    }
                    // If minor versions differ, that's our result.
                    other_ordering => other_ordering,
                }
            }
            // If major versions differ, that's our result.
            other_ordering => other_ordering,
        }
    }
}

fn main() {
    let v1 = Version {
        major: 1,
        minor: 2,
        patch: 3,
    };
    let v2 = Version {
        major: 1,
        minor: 3,
        patch: 0,
    };
    let v3 = Version {
        major: 2,
        minor: 0,
        patch: 0,
    };

    // We can sort a collection of versions:
    let mut versions = vec![v3, v1, v2];
    println!("Before sorting: {versions:?}");

    versions.sort(); // `sort()` requires the `Ord` trait.

    println!("After sorting: {versions:?}");

    // The sorted order should be [1.2.3, 1.3.0, 2.0.0].
    assert_eq!(
        versions[0],
        Version {
            major: 1,
            minor: 2,
            patch: 3
        }
    );
    assert_eq!(
        versions[1],
        Version {
            major: 1,
            minor: 3,
            patch: 0
        }
    );
    assert_eq!(
        versions[2],
        Version {
            major: 2,
            minor: 0,
            patch: 0
        }
    );
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
