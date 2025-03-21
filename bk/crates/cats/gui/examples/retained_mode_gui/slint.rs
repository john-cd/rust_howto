#![allow(dead_code)]
// ANCHOR: example
slint::slint! {
    export component HelloWorld inherits Window {
        Text {
            text: "hello world";
            color: green;
        }
    }
}

pub fn main() -> anyhow::Result<()> {
    // `run` is a convenience function that first calls Self::show, followed by
    // crate::run_event_loop() and Self::hide.
    HelloWorld::new()?.run()?;
    Ok(())
}
// ANCHOR_END: example

// [finish](https://github.com/john-cd/rust_howto/issues/787) figure how to test - neither WAYLAND_DISPLAY nor WAYLAND_SOCKET nor DISPLAY is set.
// see vs code Wayland setting!
