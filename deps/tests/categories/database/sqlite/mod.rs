#![cfg(feature = "linux_only")]

mod initialization;
mod insert_select;
mod transactions;

#[test]
fn test() -> anyhow::Result<()> {
    initialization::main()?;
    insert_select::main()?;
    transactions::main()?;
    Ok(())
}
