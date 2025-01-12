// ANCHOR: example

slint::slint! {
    export component HelloWorld {
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
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
// [P1](https://github.com/john-cd/rust_howto/issues/787)
