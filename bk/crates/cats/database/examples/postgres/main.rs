mod aggregate_data;
mod cornucopia;
mod create_tables;
mod insert_query_data;
mod tokio_postgres;

fn main() -> anyhow::Result<()> {
    create_tables::main()?;
    insert_query_data::main()?;
    // aggregate_data::main()?;
    Ok(())
}

#[test]
fn require_external_svc() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
// [review](https://github.com/john-cd/rust_howto/issues/713)
