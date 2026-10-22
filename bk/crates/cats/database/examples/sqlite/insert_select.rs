//! This function demonstrates inserting data into a SQLite database and then
//! selecting it back.
//!
//! TODO Reviewed ??-??-2026

#![allow(dead_code)]

// ANCHOR: example
// This example demonstrates inserting and selecting SQLite data using
// `rusqlite`.
use std::collections::HashMap;

use rusqlite::Connection;
use rusqlite::Result;

/// A cat.
#[derive(Debug)]
#[allow(dead_code)]
struct Cat {
    /// The name of the cat.
    name: String,
    /// The color of the cat.
    color: String,
}

/// This function demonstrates inserting data into a SQLite database and then
/// selecting it back.
fn main() -> Result<()> {
    let conn = Connection::open("temp/cats.db")?;

    // Create a HashMap to store cat colors and their corresponding names.
    let mut cat_colors = HashMap::new();
    cat_colors.insert(String::from("Blue"), vec!["Tigger", "Sammy"]);
    cat_colors.insert(String::from("Black"), vec!["Oreo", "Biscuit"]);

    // Prepare SQL statements for inserting cat colors and cats outside the
    // loop.
    let mut color_stmt =
        conn.prepare("INSERT INTO cat_colors (name) values (?1)")?;
    let mut cat_stmt =
        conn.prepare("INSERT INTO cats (name, color_id) values (?1, ?2)")?;

    for (color, catnames) in &cat_colors {
        color_stmt.execute([color])?;
        // Get the SQLite rowid of the most recent successful INSERT.
        let last_id = conn.last_insert_rowid();

        // Insert each cat name and its corresponding color_id into the cats
        // table.
        for cat in catnames {
            cat_stmt.execute(rusqlite::params![cat, last_id])?;
        }
    }

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

    for cat in cats {
        println!("Found cat {cat:?}");
    }

    Ok(())
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

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

        // Baseline approach: conn.execute inside loop
        let start = std::time::Instant::now();
        for i in 0..num_colors {
            let color = format!("Color_{i}");
            conn.execute(
                "INSERT INTO cat_colors (name) values (?1)",
                [&color],
            )?;
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

        // Clear tables
        conn.execute("DELETE FROM cats", ())?;
        conn.execute("DELETE FROM cat_colors", ())?;

        // Prepared statement approach
        let start = std::time::Instant::now();
        let mut color_stmt =
            conn.prepare("INSERT INTO cat_colors (name) values (?1)")?;
        let mut cat_stmt =
            conn.prepare("INSERT INTO cats (name, color_id) values (?1, ?2)")?;

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
}
