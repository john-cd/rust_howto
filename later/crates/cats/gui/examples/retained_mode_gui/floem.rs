// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A reactive counter application with Floem.

use floem::prelude::*;

/// Increases the counter's reactive state by one.
fn increment(counter: RwSignal<i32>) {
    counter.set(counter.get() + 1);
}

/// Decreases the counter's reactive state by one.
fn decrement(counter: RwSignal<i32>) {
    counter.set(counter.get() - 1);
}

/// Builds the counter view with fine-grained reactive signals.
fn counter_view() -> impl IntoView {
    let counter = RwSignal::new(0);

    v_stack((
        label(move || format!("Value: {}", counter.get())),
        h_stack((
            button("+").action(move || increment(counter)),
            button("-").action(move || decrement(counter)),
        )),
    ))
    .style(|s| s.size_full().items_center().justify_center().gap(10))
}

/// Starts the Floem counter window.
fn main() {
    floem::launch(counter_view);
}
// ANCHOR_END: example

/// Runs the Floem counter example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that counter actions update the signal without a window.
    #[test]
    fn actions_update_reactive_counter_state() {
        let counter = RwSignal::new(0);

        increment(counter);
        assert_eq!(counter.get(), 1);

        decrement(counter);
        assert_eq!(counter.get(), 0);
    }

    /// Runs the Floem entry point in a native window.
    #[test]
    #[ignore = "opens a window"]
    fn test_floem_example_entry_point() {
        main();
    }
}
