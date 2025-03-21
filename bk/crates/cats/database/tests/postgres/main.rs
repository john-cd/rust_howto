mod aggregate_data;
mod create_tables;
mod insert_query_data;

mod cornucopia;
mod tokio_postgres;

#[test]
fn require_external_svc() -> anyhow::Result<()> {
    create_tables::main()?;
    insert_query_data::main()?;
    // aggregate_data::main()?;
    Ok(())
}
// [review NOW](https://github.com/john-cd/rust_howto/issues/713)
