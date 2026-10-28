#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! A custom `proptest` strategy that only generates valid values.

/// A closed range of whole numbers with `start <= end`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Span {
    start: i32,
    end: i32,
}

impl Span {
    /// Returns `None` when `start > end`.
    fn new(start: i32, end: i32) -> Option<Self> {
        (start <= end).then_some(Self { start, end })
    }

    /// Returns the number of integers in the span.
    fn len(&self) -> u64 {
        u64::try_from(i64::from(self.end) - i64::from(self.start) + 1)
            .expect("`start <= end` guarantees a positive length")
    }

    /// Returns the smallest span that contains both spans.
    fn union(&self, other: &Self) -> Self {
        Self {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    /// Returns `true` if the two spans share at least one integer.
    fn overlaps(&self, other: &Self) -> bool {
        self.start <= other.end && other.start <= self.end
    }
}
// ANCHOR_END: example

/// The example runs under `cargo test`.
fn main() {}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    // `prop_compose!` builds a strategy from smaller ones. The second
    // argument depends on the first, so every generated `Span` is valid
    // and no test input is discarded.
    prop_compose! {
        /// Generates spans whose endpoints satisfy `start <= end`.
        fn any_span()(start in -1_000..1_000i32)
                     (length in 0..500i32, start in Just(start)) -> Span {
            Span::new(start, start + length).expect("valid by construction")
        }
    }

    proptest! {
        /// Checks that the union contains both input spans.
        #[test]
        fn union_contains_both(a in any_span(), b in any_span()) {
            let u = a.union(&b);
            prop_assert!(u.start <= a.start && a.end <= u.end);
            prop_assert!(u.start <= b.start && b.end <= u.end);
        }

        /// Checks that span union is commutative.
        #[test]
        fn union_is_commutative(a in any_span(), b in any_span()) {
            prop_assert_eq!(a.union(&b), b.union(&a));
        }

        /// Checks that span overlap is symmetric.
        #[test]
        fn overlap_is_symmetric(a in any_span(), b in any_span()) {
            prop_assert_eq!(a.overlaps(&b), b.overlaps(&a));
        }

        /// Checks that disjoint spans produce a union at least as large as their total length.
        #[test]
        fn disjoint_union_is_not_smaller(a in any_span(), b in any_span()) {
            prop_assume!(!a.overlaps(&b));
            prop_assert!(a.union(&b).len() >= a.len() + b.len());
        }

        // `new` rejects exactly the inverted ranges.
        /// Checks that construction succeeds exactly when endpoints are ordered.
        #[test]
        fn new_matches_ordering(start in any::<i32>(), end in any::<i32>()) {
            prop_assert_eq!(Span::new(start, end).is_some(), start <= end);
        }
    }

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
