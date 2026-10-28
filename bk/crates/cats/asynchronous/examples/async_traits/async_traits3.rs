// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Asynchronous trait methods with the `async_trait` crate.

use async_trait::async_trait;

// Define an asynchronous trait named `Advertisement`.
/// An advertisement that can be run asynchronously.
#[async_trait]
trait Advertisement {
    /// Shows the advertisement.
    async fn run(&self);
}

/// A modal advertisement.
struct Modal;

// Implement the `Advertisement` trait for the `Modal` struct:
#[async_trait]
impl Advertisement for Modal {
    async fn run(&self) {
        // Call the `render_fullscreen` method on `self` and await its
        // completion.
        self.render_fullscreen().await;
        // Loop 4 times.
        for _ in 0..4u16 {
            // Call the `remind_user_to_join_mailing_list` function and await
            // its completion.
            remind_user_to_join_mailing_list().await;
        }
        // Call the `hide_for_now` method on `self` and await its completion.
        self.hide_for_now().await;
    }
}

// Implement methods for the `Modal` struct.
impl Modal {
    /// Shows the modal in full screen.
    async fn render_fullscreen(&self) {
        println!("Render fullscreen");
    }

    /// Hides the modal for now.
    async fn hide_for_now(&self) {
        println!("Hide for now");
    }
}

/// Asks the user to join the mailing list.
async fn remind_user_to_join_mailing_list() {
    println!("Please join our mailing list");
}

/// Runs an advertisement on the Tokio runtime.
#[tokio::main]
async fn main() {
    // Create an instance of `Modal` and call the `run` method, awaiting its
    // completion:
    Modal.run().await;
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the async traits3 example through its entry point.
    #[test]
    fn test_async_traits3_example_entry_point() {
        main();
    }
}
