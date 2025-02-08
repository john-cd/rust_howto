// ANCHOR: example
use postgres::Client;
use postgres::NoTls;

pub fn main() -> anyhow::Result<()> {
    // The connection URL is formatted as
    // postgresql://<user>:<password>@<host>/<db>, for example
    // postgresql://postgres:postgres@localhost/library
    let mut client = Client::connect(
        "postgresql://postgres:mysecretpassword@rust_howto_dev-postgres-1/library",
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
    println!("Tables created!");
    Ok(())
}
// ANCHOR_END: example
