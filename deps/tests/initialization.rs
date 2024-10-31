use rusqlite::Connection;
use rusqlite::Result;

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;
    // or: let conn = Connection::open("cats.db")?;

    conn.execute(
        "create table if not exists cat_colors (
                 id integer primary key,
                 name text not null unique
             )",
        (), // empty list of parameters.
    )?;
    conn.execute(
        "create table if not exists cats (
                 id integer primary key,
                 name text not null,
                 color_id integer not null references
cat_colors(id)              )",
        (),
    )?;

    Ok(())
}

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
