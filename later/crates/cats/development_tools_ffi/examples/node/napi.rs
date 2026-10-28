// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Calling Rust from Node.js with `napi`.
#![cfg_attr(
    not(feature = "node"),
    allow(
        dead_code,
        reason = "the exports are only called through the generated addon"
    )
)]

// Framework for building pre-compiled Node.js addons in Rust.
#[cfg(feature = "node")]
use napi_derive::napi;

/// Adds two integers for a JavaScript caller.
#[cfg_attr(feature = "node", napi)]
pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

/// Represents a point passed between JavaScript and Rust.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "node", napi(object))]
pub struct Point {
    /// Horizontal coordinate.
    pub x: f64,
    /// Vertical coordinate.
    pub y: f64,
}

/// Returns a point translated by the supplied offsets.
#[cfg_attr(feature = "node", napi)]
pub fn translate_point(point: Point, offset_x: f64, offset_y: f64) -> Point {
    Point {
        x: point.x + offset_x,
        y: point.y + offset_y,
    }
}

/// Returns a greeting for `name` as a JavaScript string.
#[cfg_attr(feature = "node", napi)]
pub fn hello(name: String) -> String {
    format!("Hello, {name}!")
}

/// Returns an asynchronous greeting that becomes a JavaScript Promise.
#[cfg_attr(feature = "node", napi)]
pub async fn greet_async(name: String) -> String {
    hello(name)
}

/// Exercises the pure function before it is wrapped in a generated addon.
fn main() {
    println!("{} + {} = {}", 2, 3, sum(2, 3));
}
// ANCHOR_END: example

/// Runs the pure Rust N-API example without loading a Node runtime.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use std::future::Future;
    use std::task::Context;
    use std::task::Poll;
    use std::task::Waker;

    use super::*;

    /// Runs the napi example through its entry point.
    #[test]
    fn test_napi_example_entry_point() {
        main();
    }

    /// Verifies the same greeting that the JavaScript smoke test expects.
    #[test]
    fn greeting_includes_the_javascript_argument() {
        assert_eq!(hello("Ada".to_owned()), "Hello, Ada!");
    }

    /// Checks that the exported sum has ordinary Rust behavior.
    #[test]
    fn sum_adds_two_integers() {
        assert_eq!(sum(20, 22), 42);
    }

    /// Checks that a JavaScript point maps to a translated Rust value.
    #[test]
    fn point_translation_preserves_both_coordinates() {
        assert_eq!(
            translate_point(Point { x: 1.5, y: -2.0 }, 2.5, 4.0),
            Point { x: 4.0, y: 2.0 }
        );
    }

    /// Checks that the async export resolves to the personalized greeting.
    #[test]
    fn async_greeting_includes_the_javascript_argument() {
        let waker = Waker::noop();
        let mut context = Context::from_waker(waker);
        let mut greeting = std::pin::pin!(greet_async("Ada".to_owned()));

        assert_eq!(
            greeting.as_mut().poll(&mut context),
            Poll::Ready("Hello, Ada!".to_owned())
        );
    }
}
// [finish; https://lib.rs/crates/napi; https://github.com/napi-rs/package-template](https://github.com/john-cd/rust_howto/issues/1032)
