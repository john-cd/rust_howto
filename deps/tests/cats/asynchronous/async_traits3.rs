// ANCHOR: example
use async_trait::async_trait;

#[async_trait]
trait Advertisement {
    async fn run(&self);
}

struct Modal;

#[async_trait]
impl Advertisement for Modal {
    async fn run(&self) {
        self.render_fullscreen().await;
        for _ in 0..4u16 {
            remind_user_to_join_mailing_list().await;
        }
        self.hide_for_now().await;
    }
}

impl Modal {
    async fn render_fullscreen(&self) {}

    async fn hide_for_now(&self) {}
}

async fn remind_user_to_join_mailing_list() {}

#[tokio::main]
async fn main() {
    Modal.run().await;
}

// ANCHOR_END: example
#[test]
fn test() {
    main();
}
