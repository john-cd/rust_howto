// TODO Reviewed ??-??-2026.

// ANCHOR: example
use std::future::pending;
use std::rc::Rc;

/// Requires Send so the fixture diagnoses an Rc retained across an await.
fn require_send<T: Send>(_: T) {}

/// Runs the example demonstrated by this module.
fn main() {
    let future = async {
        let local = Rc::new(1);
        pending::<()>().await;
        drop(local);
    };
    require_send(future);
}
// ANCHOR_END: example
