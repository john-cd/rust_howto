// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This function demonstrates inserting data into a SQLite database and then
//! selecting it back.
//!
//! SQLite is a database in a single file, with no server to install. Use `?`
//! placeholders for values instead of building SQL by concatenation; it
//! prevents SQL injection, the attack where user input is interpreted as SQL.

// This example demonstrates inserting and selecting SQLite data using
// `rusqlite`.
use std::collections::HashMap;

use rusqlite::Connection;
use rusqlite::Result;

/// A cat.
#[derive(Debug)]
struct Cat {
    /// The name of the cat.
    name: String,
    /// The color of the cat.
    color: String,
}

/// Creates the `cat_colors` and `cats` tables.
///
/// # Errors
///
/// Returns an error if the tables already exist or SQLite rejects the statements.
fn create_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE cat_colors (id INTEGER PRIMARY KEY, name TEXT NOT NULL UNIQUE);
         CREATE TABLE cats (id INTEGER PRIMARY KEY, name TEXT NOT NULL, color_id INTEGER NOT NULL);",
    )
}

/// Inserts each color, then every cat of that color, using prepared statements.
///
/// # Errors
///
/// Returns an error if a color is inserted twice or the tables do not exist.
fn insert_cats(conn: &Connection, cat_colors: &HashMap<String, Vec<&str>>) -> Result<()> {
    // Prepare SQL statements for inserting cat colors and cats outside the
    // loop.
    let mut color_stmt = conn.prepare("INSERT INTO cat_colors (name) values (?1)")?;
    let mut cat_stmt = conn.prepare("INSERT INTO cats (name, color_id) values (?1, ?2)")?;

    for (color, catnames) in cat_colors {
        color_stmt.execute([color])?;
        // Get the SQLite rowid of the most recent successful INSERT.
        let last_id = conn.last_insert_rowid();

        // Insert each cat name and its corresponding color_id into the cats
        // table.
        for cat in catnames {
            cat_stmt.execute(rusqlite::params![cat, last_id])?;
        }
    }
    Ok(())
}

/// Selects every cat together with the name of its color.
///
/// # Errors
///
/// Returns an error if the tables do not exist or a row cannot be read.
fn select_cats(conn: &Connection) -> Result<Vec<Cat>> {
    // Prepare a SQL statement to select cat names and their corresponding
    // colors.
    let mut stmt = conn.prepare(
        "SELECT c.name, cc.name from cats c
         INNER JOIN cat_colors cc
         ON cc.id = c.color_id;",
    )?;

    let cats = stmt.query_map([], |row| {
        // Map each row to a Cat struct.
        Ok(Cat {
            name: row.get(0)?,
            color: row.get(1)?,
        })
    })?;

    cats.collect()
}

/// This function demonstrates inserting data into a SQLite database and then
/// selecting it back.
fn main() -> Result<Vec<Cat>> {
    let conn = Connection::open_in_memory()?;
    create_schema(&conn)?;

    // Create a HashMap to store cat colors and their corresponding names.
    let mut cat_colors = HashMap::new();
    cat_colors.insert(String::from("Blue"), vec!["Tigger", "Sammy"]);
    cat_colors.insert(String::from("Black"), vec!["Oreo", "Biscuit"]);

    insert_cats(&conn, &cat_colors)?;

    let cats = select_cats(&conn)?;
    for cat in &cats {
        println!("Found cat {cat:?}: {} is {}", cat.name, cat.color);
    }

    Ok(cats)
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() -> Result<()> {
    main().map(|_| ())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    /// Verifies that bench insert performance.
    #[test]
    fn bench_insert_performance() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        conn.execute(
            "create table cat_colors (id integer primary key, name text not null unique)",
            (),
        )?;
        conn.execute(
            "create table cats (id integer primary key, name text not null, color_id integer not null)",
            (),
        )?;

        let num_colors = 100;
        let cats_per_color = 50;

        // The baseline executes each SQL statement inside the insertion loop.
        let start = std::time::Instant::now();
        for i in 0..num_colors {
            let color = format!("Color_{i}");
            conn.execute("INSERT INTO cat_colors (name) values (?1)", [&color])?;
            let last_id: String = conn.last_insert_rowid().to_string();
            for j in 0..cats_per_color {
                let cat = format!("Cat_{i}_{j}");
                conn.execute(
                    "INSERT INTO cats (name, color_id) values (?1, ?2)",
                    [&cat, &last_id],
                )?;
            }
        }
        let baseline_duration = start.elapsed();

        // Clear both tables before measuring the prepared-statement approach.
        conn.execute("DELETE FROM cats", ())?;
        conn.execute("DELETE FROM cat_colors", ())?;

        // Prepare each statement once and reuse it throughout the loop.
        let start = std::time::Instant::now();
        let mut color_stmt = conn.prepare("INSERT INTO cat_colors (name) values (?1)")?;
        let mut cat_stmt = conn.prepare("INSERT INTO cats (name, color_id) values (?1, ?2)")?;

        for i in 0..num_colors {
            let color = format!("Color_{i}");
            color_stmt.execute([&color])?;
            let last_id = conn.last_insert_rowid();
            for j in 0..cats_per_color {
                let cat = format!("Cat_{i}_{j}");
                cat_stmt.execute(rusqlite::params![cat, last_id])?;
            }
        }
        let optimized_duration = start.elapsed();

        eprintln!(
            "\n--- Benchmark Results ---\nBaseline duration (conn.execute): {:?}\nOptimized duration (prepared stmt): {:?}\nSpeedup: {:.2}x\n-------------------------",
            baseline_duration,
            optimized_duration,
            baseline_duration.as_secs_f64() / optimized_duration.as_secs_f64()
        );

        Ok(())
    }

    /// An empty input inserts nothing and selects nothing.
    #[test]
    fn insert_and_select_with_no_cats_is_empty() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        create_schema(&conn)?;
        insert_cats(&conn, &HashMap::new())?;
        assert!(select_cats(&conn)?.is_empty());
        Ok(())
    }

    /// A color without cats leaves no joined rows.
    #[test]
    fn color_without_cats_is_not_selected() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        create_schema(&conn)?;
        let colors = HashMap::from([(String::from("Grey"), vec![])]);
        insert_cats(&conn, &colors)?;
        assert!(select_cats(&conn)?.is_empty());
        Ok(())
    }

    /// Creating the schema twice fails instead of silently succeeding.
    #[test]
    fn creating_the_schema_twice_is_an_error() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        create_schema(&conn)?;
        assert!(create_schema(&conn).is_err());
        Ok(())
    }

    /// Inserting before the schema exists reports an error.
    #[test]
    fn inserting_without_a_schema_is_an_error() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        let colors = HashMap::from([(String::from("Blue"), vec!["Tigger"])]);
        assert!(insert_cats(&conn, &colors).is_err());
        Ok(())
    }

    #[test]
    fn test_main_entry_point() -> Result<()> {
        let cats = main()?;
        assert_eq!(cats.len(), 4);
        let actual = cats
            .into_iter()
            .map(|cat| (cat.name, cat.color))
            .collect::<BTreeSet<_>>();
        assert_eq!(
            actual,
            BTreeSet::from([
                ("Biscuit".to_owned(), "Black".to_owned()),
                ("Oreo".to_owned(), "Black".to_owned()),
                ("Sammy".to_owned(), "Blue".to_owned()),
                ("Tigger".to_owned(), "Blue".to_owned()),
            ])
        );
        Ok(())
    }
}
