//! Selects a standard-library API by compiler version.

// ANCHOR: example

/// Flips a flag using the standard-library method introduced in Rust 1.100.
#[rustversion::since(1.100)]
fn flip(flag: &mut bool) {
    flag.toggle();
}

/// Flips a flag on compilers that predate `bool::toggle`.
#[rustversion::before(1.100)]
fn flip(flag: &mut bool) {
    *flag = !*flag;
}

/// Exercises the implementation selected by the compiler version.
fn main() {
    let mut flag = false;
    flip(&mut flag);
    assert!(flag);
    flip(&mut flag);
    assert!(!flag);
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    /// Checks that the selected implementation flips the flag in both directions.
    #[test]
    fn flips_in_both_directions() {
        super::main();
    }
}
