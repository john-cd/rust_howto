use rusqlite::Connection;
use rusqlite::Result;
use rusqlite::NO_PARAMS;

fn main() -> Result<()> {
    let mut conn = Connection::open("cats.db")?;

    successful_tx(&mut conn)?;

    let res = rolled_back_tx(&mut conn);
    assert!(res.is_err());

    Ok(())
}

fn successful_tx(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;

    tx.execute("delete from cat_colors", NO_PARAMS)?;
    tx.execute("insert into cat_colors (name) values (?1)", &[&"lavender"])?;
    tx.execute(
        "insert into cat_colors (name)
values (?1)",
        &[&"blue"],
    )?;

    tx.commit()
}

fn rolled_back_tx(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;

    tx.execute("delete from cat_colors", NO_PARAMS)?;
    tx.execute("insert into cat_colors (name) values (?1)", &[&"lavender"])?;
    tx.execute(
        "insert into cat_colors (name)
values (?1)",
        &[&"blue"],
    )?;
    tx.execute(
        "insert into cat_colors
(name) values (?1)",
        &[&"lavender"],
    )?;

    tx.commit()
}
