#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! A counter application with `floem`.

use floem::prelude::*;

/// Runs the floem example.
pub fn main() {
    floem::launch(counter_view);
}

/// Returns the view of the counter.
fn counter_view() -> impl IntoView {
    let counter = RwSignal::new(0);

    h_stack((
        button("Increment").action(move || counter.set(counter.get() + 1)),
        label(move || format!("Value: {}", counter.get())),
        button("Decrement").action(move || counter.set(counter.get() - 1)),
    ))
    .style(|s| s.size_full().items_center().justify_center().gap(10))
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "requires an interactive example runtime"]
    fn test_main() {
        main();
    }
}
