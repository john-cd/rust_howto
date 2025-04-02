// ANCHOR: example
use async_trait::async_trait;

// Define an asynchronous trait named `Advertisement`.
#[async_trait]
trait Advertisement {
    async fn run(&self);
}

struct Modal;

// Implement the `Advertisement` trait for the `Modal` struct.
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
    async fn render_fullscreen(&self) {
        println!("Render fullscreen");
    }

    async fn hide_for_now(&self) {
        println!("Hide for now");
    }
}

async fn remind_user_to_join_mailing_list() {
    println!("Please join our mailing list");
}

#[tokio::main]
async fn main() {
    // Create an instance of `Modal` and call the `run` method, awaiting its
    // completion.
    Modal.run().await;
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
