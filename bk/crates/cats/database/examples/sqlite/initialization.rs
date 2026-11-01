// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates how to initialize a SQLite database with tables.
//!
//! SQLite is a database in a single file, with no server to install; it is
//! built into phones, browsers and many applications. `CREATE TABLE IF NOT
//! EXISTS` makes setup safe to run more than once.

use std::fs;

use rusqlite::Connection;

/// Creates a SQLite database and a table.
fn main() -> anyhow::Result<()> {
    // Create the "temp" directory if it doesn't exist.
    if !fs::exists("temp")? {
        fs::create_dir("temp")?;
    }

    let conn = Connection::open("temp/cats.db")?;

    create_tables(&conn)?;

    println!("Tables created.");
    Ok(())
}

/// Creates the tables used by the SQLite cat example.
///
/// # Errors
///
/// Returns an error if SQLite cannot create either table.
fn create_tables(conn: &Connection) -> anyhow::Result<()> {
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
    Ok(())
}
// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies both example tables can be created in an in-memory database.
    #[test]
    fn test_sqlite_initialization_creates_both_tables() -> anyhow::Result<()> {
        let conn = Connection::open_in_memory()?;
        create_tables(&conn)?;

        let table_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name IN ('cat_colors', 'cats')",
            [],
            |row| row.get(0),
        )?;
        assert_eq!(table_count, 2);
        Ok(())
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
