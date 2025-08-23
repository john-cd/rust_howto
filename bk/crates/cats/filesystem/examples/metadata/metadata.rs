#![allow(dead_code)]
// ANCHOR: example
fn main() -> anyhow::Result<()> {
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
