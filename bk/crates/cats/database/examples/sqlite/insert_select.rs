#![allow(dead_code)]
// ANCHOR: example
use std::collections::HashMap;

use rusqlite::Connection;
use rusqlite::Result;

#[derive(Debug)]
#[allow(dead_code)]
struct Cat {
    name: String,
    color: String,
}

/// This function demonstrates inserting data into a SQLite database and then
/// selecting it back.
pub fn main() -> Result<()> {
    let conn = Connection::open("temp/cats.db")?;

    // Create a HashMap to store cat colors and their corresponding names.
    let mut cat_colors = HashMap::new();
    cat_colors.insert(String::from("Blue"), vec!["Tigger", "Sammy"]);
    cat_colors.insert(String::from("Black"), vec!["Oreo", "Biscuit"]);

    for (color, catnames) in &cat_colors {
        conn.execute(
            "INSERT INTO cat_colors (name) values (?1)",
            [&color.to_string()],
        )?;
        // Get the SQLite rowid of the most recent successful INSERT.
        let last_id: String = conn.last_insert_rowid().to_string();

        // Insert each cat name and its corresponding color_id into the cats
        // table.
        for cat in catnames {
            conn.execute(
                "INSERT INTO cats (name, color_id) values (?1, ?2)",
                [&cat.to_string(), &last_id],
            )?;
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
