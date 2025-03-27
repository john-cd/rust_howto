#![cfg(target_os = "linux")]

mod initialization;
mod insert_select;
mod transactions;

#[test]
fn test() -> anyhow::Result<()> {
    let _ = std::fs::remove_file("temp/cats.db");
    initialization::main()?;
    insert_select::main()?;
    transactions::main()?;
    Ok(())
}
