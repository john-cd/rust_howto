#![allow(dead_code)]
// ANCHOR: example
use floem::prelude::*;

pub fn main() {
    floem::launch(counter_view);
}

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
