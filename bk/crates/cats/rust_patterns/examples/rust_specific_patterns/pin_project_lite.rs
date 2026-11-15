// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! `pin-project-lite` projects a pinned field with a small declarative macro.
//! Unlike `pin-project`, it does not provide a custom pinned-drop attribute.

use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use std::task::Waker;

use pin_project_lite::pin_project;

pin_project! {
    /// A future whose inner future stays pinned while its counter can move.
    struct CountPolls<F> {
        #[pin]
        inner: F,
        polls: usize,
    }
}

impl<F: Future> Future for CountPolls<F> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        *this.polls += 1;
        this.inner.poll(cx)
    }
}

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

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    #[test]
    fn projected_future_completes() {
        super::main();
    }
}
