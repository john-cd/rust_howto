// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Calling Rust from Flutter with `flutter_rust_bridge`.
//!
//! Flutter builds mobile and desktop interfaces in Dart. `flutter_rust_bridge`
//! generates the glue so Dart can call Rust functions with ordinary types,
//! which is useful for heavy computation or sharing code with other platforms.

// Integrate a Rust function into a Flutter project using
// `flutter_rust_bridge`.
// Typical scenarios to combine them include:
//
// - When you want a UI framework for your Rust system.
// - Use arbitrary Rust libraries in Flutter: When the desired
// functionality only has a library in Rust, not Dart / Flutter.
// - Need high-performance code for Flutter: Rust makes it easy and
// performant to write multi-thread code, data-intensive operations.
//
// 1. Install Rust and [Flutter](https://flutter.dev); Flutter includes Dart.
// 2. Install the code generator at the same version as the Rust runtime.
// ```sh
// cargo binstall --version 2.13.0 flutter_rust_bridge_codegen
// ```
// 3. Create and run a Flutter project with a Rust bridge.
// ```sh
// flutter_rust_bridge_codegen create my_app
// cd my_app && flutter_rust_bridge_codegen generate && flutter run
// ```
// 4. Regenerate the bridge whenever the Rust API changes:
// ```sh
// flutter_rust_bridge_codegen generate && flutter run
// ```
// 5. Automatically regenerate the bridge when Rust code changes:
// `flutter_rust_bridge_codegen generate --watch`

/// Greets a user from a generated Flutter bridge.
pub fn greet(name: String) -> String {
    format!("Hello, {name}! This is Rust speaking.")
}

/// Demonstrates an asynchronous Rust function exposed to Flutter.
pub async fn greet_async(name: String) -> String {
    greet(name)
}

/// Builds the values sent by the stream example so the API can be tested
/// without Flutter.
fn greeting_updates(name: String) -> impl Iterator<Item = String> {
    [
        greet(name.clone()),
        format!("Welcome, {name}! You are receiving updates from Rust."),
    ]
    .into_iter()
}

/// Streams greeting updates to a Dart `Stream` and returns send failures to the
/// caller.
///
/// # Errors
///
/// Returns an error if the bridge cannot send a value to Dart.
#[cfg(feature = "flutter")]
pub fn greet_stream(
    name: String,
    sink: crate::frb_generated::StreamSink<String>,
) -> anyhow::Result<()> {
    for update in greeting_updates(name) {
        sink.add(update)
            .map_err(|error| anyhow::anyhow!(error.to_string()))?;
    }
    Ok(())
}

/// Exercises the same plain Rust API that a generated Flutter bridge calls.
fn main() {
    for update in greeting_updates("Flutter".to_owned()) {
        println!("{update}");
    }
}

// ANCHOR_END: example

/// Runs the Flutter Rust Bridge example.
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

    /// Runs the flutter rust bridge example through its entry point.
    #[test]
    fn test_flutter_rust_bridge_example_entry_point() {
        main();
    }

    /// Checks that the bridge API preserves the supplied name.
    #[test]
    fn greeting_includes_the_supplied_name() {
        assert_eq!(
            greet("Ada".to_owned()),
            "Hello, Ada! This is Rust speaking."
        );
    }

    /// Checks that the asynchronous bridge API returns the same greeting.
    #[test]
    fn async_greeting_includes_the_supplied_name() {
        let waker = Waker::noop();
        let mut context = Context::from_waker(waker);
        let mut greeting = std::pin::pin!(greet_async("Ada".to_owned()));

        assert_eq!(
            greeting.as_mut().poll(&mut context),
            Poll::Ready("Hello, Ada! This is Rust speaking.".to_owned())
        );
    }

    /// Checks that the stream API sends useful updates in a stable order.
    #[test]
    fn greeting_updates_are_ordered_and_personalized() {
        assert_eq!(
            greeting_updates("Ada".to_owned()).collect::<Vec<_>>(),
            vec![
                "Hello, Ada! This is Rust speaking.".to_owned(),
                "Welcome, Ada! You are receiving updates from Rust.".to_owned(),
            ]
        );
    }
}
