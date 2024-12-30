// ANCHOR: example
use std::fs;

use anyhow::Result;
use rusqlite::Connection;

pub fn main() -> Result<()> {
    if !fs::exists("temp")? {
        fs::create_dir("temp")?;
    }
    let mut conn = Connection::open("temp/cats.db")?;

    successful_tx(&mut conn)?;
    println!("Successful transaction.");

    let res = rolled_back_tx(&mut conn);
    assert!(res.is_err());
    println!(
        "Attempt to insert the same name in a unique column fails. The transaction was rolled-back."
    );

    Ok(())
}

fn successful_tx(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;
    tx.execute("delete from cats; delete from cat_colors", ())?;
    tx.execute("insert into cat_colors (name) values (?1)", [&"lavender"])?;
    tx.execute("insert into cat_colors (name) values (?1)", [&"blue"])?;
    tx.commit()?;
    Ok(())
}

fn rolled_back_tx(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;
    tx.execute("delete from cats; delete from cat_colors", ())?;
    tx.execute("insert into cat_colors (name) values (?1)", [&"lavender"])?;
    tx.execute("insert into cat_colors (name) values (?1)", [&"blue"])?;
    tx.execute("insert into cat_colors (name) values (?1)", [&"lavender"])?;
    tx.commit()?;
    Ok(())
}
// ANCHOR_END: example
