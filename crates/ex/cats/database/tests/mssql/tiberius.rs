// ANCHOR: example
fn main() -> anyhow::Result<()> {
    Ok(())
}
// ANCHOR_END: example

#[test]
#[ignore = "not implemented yet"]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
// TODO P2 write tiberius example
