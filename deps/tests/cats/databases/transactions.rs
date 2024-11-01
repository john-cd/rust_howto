// ANCHOR: example
// // TODO Error: SqliteFailure(Error { code: Unknown, extended_code:
// 1 }, Some("no such table: cat_colors"))

// use rusqlite::Connection;
// use rusqlite::Result;

// fn main() -> Result<()> {
//     let mut conn = Connection::open("cats.db")?;

//     successful_tx(&mut conn)?;

//     let res = rolled_back_tx(&mut conn);
//     assert!(res.is_err());

//     Ok(())
// }

// fn successful_tx(conn: &mut Connection) -> Result<()> {
//     let tx = conn.transaction()?;

//     tx.execute("delete from cat_colors", ())?;
//     tx.execute("insert into cat_colors (name) values (?1)",
// &[&"lavender"])?;     tx.execute("insert into cat_colors (name)
// values (?1)", &[&"blue"])?;

//     tx.commit()
// }

// fn rolled_back_tx(conn: &mut Connection) -> Result<()> {
//     let tx = conn.transaction()?;

//     tx.execute("delete from cat_colors", ())?;
//     tx.execute("insert into cat_colors (name) values (?1)",
// &[&"lavender"])?;     tx.execute(
//         "insert into cat_colors (name) values (?1)",
//         &[&"blue"],
//     )?;
//     tx.execute(
//         "insert into cat_colors (name) values (?1)",
//         &[&"lavender"],
//     )?;

//     tx.commit()
// }

fn main() -> anyhow::Result<()> {
    Ok(())
}

// ANCHOR_END: example
// TODO
#[ignore]
#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
