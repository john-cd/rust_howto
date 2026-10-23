#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Build `SeaORM` queries from an inline entity definition.
//!
//! This example creates a simple active model and prints the generated SQL
//! for an insert and select statement.

use sea_orm::ActiveValue::NotSet;
use sea_orm::ActiveValue::Set;
use sea_orm::ColumnTrait as _;
use sea_orm::DbBackend;
use sea_orm::DbErr;
use sea_orm::EntityTrait as _;
use sea_orm::QueryFilter as _;
use sea_orm::QueryTrait as _;

/// The entity of the `posts` table.
mod post {
    use sea_orm::entity::prelude::*;

    /// A row of the `posts` table.
    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "posts")]
    pub struct Model {
        /// The identifier of the post.
        #[sea_orm(primary_key)]
        pub id: i32,
        /// The title of the post.
        pub title: String,
        /// The content of the post.
        pub content: String,
    }

    /// The relations of the entity, which has none.
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Builds insert and select queries and prints the SQL.
fn main() -> Result<(), DbErr> {
    let insert_sql = post::Entity::insert(post::ActiveModel {
        id: NotSet,
        title: Set("Hello SeaORM".to_owned()),
        content: Set("Building SQL without opening a database".to_owned()),
    })
    .build(DbBackend::Sqlite)
    .to_string();

    let select_sql = post::Entity::find()
        .filter(post::Column::Title.contains("SeaORM"))
        .build(DbBackend::Sqlite)
        .to_string();

    println!("INSERT SQL: {insert_sql}");
    println!("SELECT SQL: {select_sql}");
    Ok(())
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() -> Result<(), DbErr> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() -> Result<(), DbErr> {
        main()
    }
    // [finish](https://github.com/john-cd/rust_howto/issues/715)
}
