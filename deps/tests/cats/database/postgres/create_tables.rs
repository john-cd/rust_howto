// ANCHOR: example
use postgres::Client;
use postgres::Error;
use postgres::NoTls;

fn main() -> Result<(), Error> {
    let mut client = Client::connect(
        "postgresql://postgres:postgres@localhost/library",
        NoTls,
    )?;

    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS author (
            id              SERIAL PRIMARY KEY,
            name            VARCHAR NOT NULL,
            country         VARCHAR NOT NULL
            )
    ",
    )?;

    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS book  (
            id              SERIAL PRIMARY KEY,
            title           VARCHAR NOT NULL,
            author_id       INTEGER NOT NULL REFERENCES author
            )
    ",
    )?;

    Ok(())
}
// ANCHOR_END: example

// TODO P2 docker compose
#[ignore = "requires a running postgres instance"]
#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
