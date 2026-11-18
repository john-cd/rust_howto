// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! `pin-project-lite` projects a pinned field with a small declarative macro.
//!
//! `Pin<&mut Self>` prevents moving a value whose address must remain stable.
//! Calling `project` exposes `inner` as a pinned mutable reference, while the
//! ordinary `polls` counter remains mutable. This lets the wrapper update its
//! bookkeeping without moving the inner future. Unlike `pin-project`, this
//! lightweight crate does not provide a custom pinned-drop attribute.

use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use std::task::Waker;

use pin_project_lite::pin_project;

pin_project! {

    /// A future whose inner future stays pinned while its counter can move.
    struct CountPolls<F> {
        // Future projected as Pin<&mut F> so polling cannot move it.
        #[pin]
        inner: F,
        // Number of poll calls forwarded to the inner future, including Pending.
        polls: usize,
    }
}

impl<F: Future> Future for CountPolls<F> {
    /// Alias for `F::Output` used to represent output.
    type Output = F::Output;

    /// Increments the movable counter and polls the structurally pinned inner future.
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        *this.polls += 1;
        this.inner.poll(cx)
    }
}

/// Runs the example demonstrated by this module.
fn main() {
    let mut future = Box::pin(CountPolls {
        inner: std::future::ready(42),
        polls: 0,
    });
    let mut cx = Context::from_waker(Waker::noop());
    assert_eq!(future.as_mut().poll(&mut cx), Poll::Ready(42));
    assert_eq!(future.polls, 1);
    println!("Future polled {} time(s)", future.polls);
}
// ANCHOR_END: example

/// Runs the example logic exposed by this module.
pub fn run() {
    main();
}

/// Tests behavior implemented by the surrounding module.
#[cfg(test)]
mod tests {
    /// Checks that projected future completes.
    #[test]
    fn projected_future_completes() {
        super::main();
    }
}
