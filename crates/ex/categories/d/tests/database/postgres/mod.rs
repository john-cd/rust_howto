mod aggregate_data;
mod create_tables;
mod insert_query_data;
// mod deadpool2;

#[test]
fn require_external_svc() -> anyhow::Result<()> {
    create_tables::main()?;
    insert_query_data::main()?;
    // aggregate_data::main()?;
    Ok(())
}
// [P0](https://github.com/john-cd/rust_howto/issues/713)
