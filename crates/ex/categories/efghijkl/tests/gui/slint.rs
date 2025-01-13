// ANCHOR: example

slint::slint! {
    export component HelloWorld inherits Window {
        Text {
            text: "hello world";
            color: green;
        }
    }
}

fn main() -> anyhow::Result<()> {
    // `run` is a convenience function that first calls Self::show, followed by
    // crate::run_event_loop() and Self::hide.
    HelloWorld::new()?.run()?;
    Ok(())
}
// ANCHOR_END: example

// #[test]
// fn test() -> anyhow::Result<()> {
//     main()?;
//     Ok(())
// }
// [P1](https://github.com/john-cd/rust_howto/issues/787)
// TODO figure how to test - neither WAYLAND_DISPLAY nor WAYLAND_SOCKET nor
// DISPLAY is set.
