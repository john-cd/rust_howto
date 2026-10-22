#![allow(dead_code)]

// TODO Reviewed ??-??-2026
// ANCHOR: example
//! A window defined with `slint`.

slint::slint! {
    export component HelloWorld inherits Window {
        Text {
            text: "hello world";
            color: green;
        }
    }
}

/// Shows the window and runs the event loop.
fn main() -> anyhow::Result<()> {
    // `run` is a convenience function that first calls Self::show, followed by
    // crate::run_event_loop() and Self::hide.
    HelloWorld::new()?.run()?;
    Ok(())
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "requires interactive example runtime"]
    fn test_main() {
        main().unwrap();
    }
}
// [finish; figure how to test - neither WAYLAND_DISPLAY nor WAYLAND_SOCKET nor DISPLAY is set. see vs code Wayland setting](https://github.com/john-cd/rust_howto/issues/787)
