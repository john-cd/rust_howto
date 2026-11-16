// TODO Reviewed ??-??-2026.

// ANCHOR: example
use std::future::pending;
use std::rc::Rc;

fn require_send<T: Send>(_: T) {}

fn main() {
    let future = async {
        let local = Rc::new(1);
        pending::<()>().await;
        drop(local);
    };
    require_send(future);
}
// ANCHOR_END: example
