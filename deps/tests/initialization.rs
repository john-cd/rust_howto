// use rusqlite::Connection;
// use rusqlite::Result;
// use rusqlite::NO_PARAMS;

// #[test]
// #[ignore]
// fn test() -> Result<()> {
//     let conn = Connection::open("cats.db")?;

//     conn.execute(
//         "create table if not exists cat_colors (
//                  id integer primary key,
//                  name text not null unique
//              )",
//         NO_PARAMS,
//     )?;
//     conn.execute(
//         "create table if not exists cats (
//                  id integer primary key,
//                  name text not null,
//                  color_id integer not null references
// cat_colors(id)              )",
//         NO_PARAMS,
//     )?;

//     Ok(())
// }
