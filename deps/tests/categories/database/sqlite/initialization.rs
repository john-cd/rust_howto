// ANCHOR: example
use rusqlite::Connection;
use rusqlite::Result;

pub fn main() -> Result<()> {
    let conn = Connection::open("temp/cats.db")?;

    conn.execute(
        "create table if not exists cat_colors (
                 id integer primary key,
                 name text not null unique
             )",
        (), // Empty list of parameters.
    )?;
    conn.execute(
        "create table if not exists cats (
                 id integer primary key,
                 name text not null,
                 color_id integer not null references cat_colors(id)
                 )",
        (), // Empty list of parameters.
    )?;
    println!("Tables created.");
    Ok(())
}
// ANCHOR_END: example
